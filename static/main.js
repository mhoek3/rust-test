window.TermApp = (function(){
    const add_button = document.getElementById('add_term');
    const modal = document.getElementById('modal');

    let term_kinds = null;
    let terms = null;

    function sleep(ms) {
        return new Promise(resolve => setTimeout(resolve, ms));
    }

    function find_term_kind( kind ) {
        return term_kinds.find(item => item.id == kind);
    }

    function find_term( term_id ) {
        return terms.find(item => item.id == term_id);
    }

    async function fetch_terms() {
        const res = await fetch('/get_terms');
        const data = await res.json();
        
        terms = data;
        render_terms( data );
    }

    async function fetch_term_kinds() {
        const res = await fetch('/get_term_kinds');
        const data = await res.json();
        
        term_kinds = data;
        console.log(term_kinds);
    }

    async function render_terms( data ) {
        const items = data;
        const table = document.getElementById('terms');

        table.innerHTML = "";

        // wait till kinds have loaded
        while (!term_kinds)
        {
            await sleep(500);
        }
        console.log(`found ${term_kinds.length} term kinds`);

        items.forEach(item => {
            console.log(item);

            const row = document.createElement('tr');
        
            // name
            const td_name = document.createElement('td');
            td_name.textContent = item.name;
            
            // kind
            const td_kind = document.createElement('td');
            const kind = find_term_kind( item.term_kind );
            td_kind.textContent = kind ? kind.name : 'no kind';

            // actions
            const td_actions = document.createElement('td');
            const btn_edit = document.createElement('button');
            btn_edit.textContent = 'Edit';
            btn_edit.onclick = () => edit_term(item.id);
            btn_edit.setAttribute('class', 'blue');

            td_actions.appendChild(btn_edit);

            row.appendChild(td_name);
            row.appendChild(td_kind);
            row.appendChild(td_actions);

            table.appendChild(row);
        });
    }

    function open_modal(){
        modal.classList.add("active");
    }

    function close_modal(){
        modal.classList.remove("active");
    }

    function edit_term( term_id ) {
        open_term_form(async function(payload){
            try {
                const res = await fetch('/edit_term', {
                    method: 'POST',
                    headers: { 'Content-Type': 'application/json' },
                    body: JSON.stringify(payload)
                });

                if (res.ok) {
                    console.log("Successfully modified");

                    fetch_terms();
                    close_modal();
                } else {
                    console.log("Error: Edit failed!");
                }
            } catch (err) {
                console.log("Network error: " + err);
            }
        }, term_id);
    }

    function add_term() {
        open_term_form(async function(payload){
            console.log(payload);

            try {
                const res = await fetch('/add_term', {
                    method: 'POST',
                    headers: { 'Content-Type': 'application/json' },
                    body: JSON.stringify(payload)
                });

                if (res.ok) {
                    console.log("Successfully added");

                    fetch_terms();
                    close_modal();
                } else {
                    console.log("Error: Adding failed!");
                }
            } catch (err) {
                console.log("Network error: " + err);
            }
        });
    }

    async function open_term_form( callback, term_id = false )
    {
        const res = await fetch('/get_term_form');
        const formHtml = await res.text();

        open_modal();

        // get form template
        const content = modal.querySelector('.content')
        content.innerHTML = `${formHtml}`;

        // reference the form element
        const form = document.getElementById('term_form');
        
        // set kind select options
        const kind_selector = form.querySelector('#kinds')
        term_kinds.forEach(kind => {
            const option = document.createElement('option');
            option.setAttribute('value', kind.id);
            option.textContent = kind.name;
            kind_selector.appendChild(option);
        });

        // set current term data in edit mode
        const term = find_term( term_id )
        if ( term ) {
            form.elements['name'].value = term.name;
            form.elements['details'].value = term.details;

            form.elements['term_kind'].value = term.term_kind;                
        }
        
        // handle form submission
        form.onsubmit = async (event) => {
            event.preventDefault();

            const formData = new FormData(form);
            const payload = {
                id          : term_id || 0,
                term_kind    : parseInt(formData.get('term_kind')),
                name        : formData.get('name'),
                details     : formData.get('details')
            };

            console.log(payload);
            callback(payload)
        };
    }
    
    async function init() {
        add_button.onclick = async () => {
            add_term();
        };

        modal.querySelector('.exit').onclick = async () => {
            close_modal();
        };

        await fetch_term_kinds();
        await fetch_terms();
    } 

    return {
        init
    }
})();