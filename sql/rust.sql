-- phpMyAdmin SQL Dump
-- version 5.2.3
-- https://www.phpmyadmin.net/
--
-- Host: db:3306
-- Gegenereerd op: 07 apr 2026 om 14:29
-- Serverversie: 8.0.45
-- PHP-versie: 8.3.30

SET SQL_MODE = "NO_AUTO_VALUE_ON_ZERO";
START TRANSACTION;
SET time_zone = "+00:00";


/*!40101 SET @OLD_CHARACTER_SET_CLIENT=@@CHARACTER_SET_CLIENT */;
/*!40101 SET @OLD_CHARACTER_SET_RESULTS=@@CHARACTER_SET_RESULTS */;
/*!40101 SET @OLD_COLLATION_CONNECTION=@@COLLATION_CONNECTION */;
/*!40101 SET NAMES utf8mb4 */;

--
-- Database: `rs_db`
--

-- --------------------------------------------------------

--
-- Tabelstructuur voor tabel `meanings`
--

CREATE TABLE `meanings` (
  `id` int UNSIGNED NOT NULL,
  `group_id` int UNSIGNED NOT NULL DEFAULT '0',
  `name` varchar(255) NOT NULL,
  `details` text NOT NULL,
  `date_created` datetime NOT NULL DEFAULT CURRENT_TIMESTAMP
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_0900_ai_ci;

--
-- Gegevens worden geëxporteerd voor tabel `meanings`
--

INSERT INTO `meanings` (`id`, `group_id`, `name`, `details`, `date_created`) VALUES
(1, 1, 'test', 'test', '2026-04-07 12:43:19'),
(2, 1, 'test 2', 'test 2', '2026-04-07 12:43:19'),
(3, 2, 'test 3', 'test 3', '2026-04-07 12:44:36'),
(4, 2, 'test 4', 'test 4', '2026-04-07 12:44:36'),
(5, 1, 'test 5', 'test 5', '2026-04-07 13:07:24'),
(6, 2, 'test 6', 'test 6', '2026-04-07 13:10:00'),
(7, 1, 'test 7', 'test 7', '2026-04-07 13:21:13');

-- --------------------------------------------------------

--
-- Tabelstructuur voor tabel `meaning_group`
--

CREATE TABLE `meaning_group` (
  `id` int UNSIGNED NOT NULL,
  `name` varchar(255) NOT NULL
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_0900_ai_ci;

--
-- Gegevens worden geëxporteerd voor tabel `meaning_group`
--

INSERT INTO `meaning_group` (`id`, `name`) VALUES
(1, 'Group 1'),
(2, 'Group 2');

--
-- Indexen voor geëxporteerde tabellen
--

--
-- Indexen voor tabel `meanings`
--
ALTER TABLE `meanings`
  ADD PRIMARY KEY (`id`);

--
-- Indexen voor tabel `meaning_group`
--
ALTER TABLE `meaning_group`
  ADD PRIMARY KEY (`id`);

--
-- AUTO_INCREMENT voor geëxporteerde tabellen
--

--
-- AUTO_INCREMENT voor een tabel `meanings`
--
ALTER TABLE `meanings`
  MODIFY `id` int UNSIGNED NOT NULL AUTO_INCREMENT, AUTO_INCREMENT=8;

--
-- AUTO_INCREMENT voor een tabel `meaning_group`
--
ALTER TABLE `meaning_group`
  MODIFY `id` int UNSIGNED NOT NULL AUTO_INCREMENT, AUTO_INCREMENT=3;
COMMIT;

/*!40101 SET CHARACTER_SET_CLIENT=@OLD_CHARACTER_SET_CLIENT */;
/*!40101 SET CHARACTER_SET_RESULTS=@OLD_CHARACTER_SET_RESULTS */;
/*!40101 SET COLLATION_CONNECTION=@OLD_COLLATION_CONNECTION */;
