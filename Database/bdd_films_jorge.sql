-- phpMyAdmin SQL Dump
-- version 5.1.1
-- https://www.phpmyadmin.net/
--
-- Hôte : localhost
-- Généré le : mar. 09 mai 2023 à 16:47
-- Version du serveur : 10.4.22-MariaDB
-- Version de PHP : 7.4.27

CREATE DATABASE bdd_films_jorge CHARACTER SET utf8mb4 COLLATE utf8mb4_unicode_ci;
use bdd_films_jorge;

SET SQL_MODE = "NO_AUTO_VALUE_ON_ZERO";
START TRANSACTION;
SET time_zone = "+00:00";


/*!40101 SET @OLD_CHARACTER_SET_CLIENT=@@CHARACTER_SET_CLIENT */;
/*!40101 SET @OLD_CHARACTER_SET_RESULTS=@@CHARACTER_SET_RESULTS */;
/*!40101 SET @OLD_COLLATION_CONNECTION=@@COLLATION_CONNECTION */;
/*!40101 SET NAMES utf8mb4 */;

--
-- Base de données : `tt`
--

-- --------------------------------------------------------

--
-- Structure de la table `acteurs`
--

CREATE TABLE `acteurs` (
  `cast_actor_id` int(11) NOT NULL,
  `nom` varchar(255) DEFAULT NULL,
  `personnage` varchar(255) DEFAULT NULL
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4;

--
-- Déchargement des données de la table `acteurs`
--

INSERT INTO `acteurs` (`cast_actor_id`, `nom`, `personnage`) VALUES
(1, 'Tobey Maguire', 'Peter Parker'),
(2, 'Willem Dafoe', 'Green Goblin'),
(3, 'Marlon Brando', 'Don Vito Corleone'),
(4, 'Al Pacino', 'Michael Corleone'),
(5, 'Leonardo DiCaprio', 'Jack Dawson'),
(6, 'Kate Winslet', 'Rose DeWitt Bukater'),
(7, 'Mark Hamill', 'Luke Skywalker'),
(8, 'Harrison Ford', 'Han Solo'),
(9, 'Tim Robbins', 'Andy Dufresne'),
(10, 'Morgan Freeman', 'Ellis Boyd \"Red\" Redding');

-- --------------------------------------------------------

--
-- Structure de la table `films`
--

CREATE TABLE `films` (
  `film_id` int(11) NOT NULL,
  `slug_film_id` varchar(255) DEFAULT NULL,
  `titre` varchar(255) DEFAULT NULL,
  `annee` int(11) DEFAULT NULL,
  `note` float DEFAULT NULL,
  `duree` int(11) DEFAULT NULL
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4;

--
-- Déchargement des données de la table `films`
--

INSERT INTO `films` (`film_id`, `slug_film_id`, `titre`, `annee`, `note`, `duree`) VALUES
(1, 'spiderman-1', 'Spider-Man', 2002, 7.3, 121),
(2, 'godfather-1', 'The Godfather', 1972, 9.2, 175),
(3, 'titanic-1', 'Titanic', 1997, 7.8, 194),
(4, 'starwars-4', 'Star Wars: Episode IV - A New Hope', 1977, 8.6, 121),
(5, 'shawshank-1', 'The Shawshank Redemption', 1994, 9.3, 142);

-- --------------------------------------------------------

--
-- Structure de la table `film_acteurs`
--

CREATE TABLE `film_acteurs` (
  `film_id` int(11) DEFAULT NULL,
  `cast_actor_id` int(11) DEFAULT NULL
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4;

--
-- Déchargement des données de la table `film_acteurs`
--

INSERT INTO `film_acteurs` (`film_id`, `cast_actor_id`) VALUES
(1, 1),
(1, 2),
(2, 3),
(2, 4),
(3, 5),
(3, 6),
(4, 7),
(4, 8),
(5, 9),
(5, 10);

-- --------------------------------------------------------

--
-- Structure de la table `genres`
--

CREATE TABLE `genres` (
  `film_id` int(11) DEFAULT NULL,
  `genre` varchar(255) DEFAULT NULL
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4;

--
-- Déchargement des données de la table `genres`
--

INSERT INTO `genres` (`film_id`, `genre`) VALUES
(1, 'Action'),
(1, 'Adventure'),
(2, 'Crime'),
(2, 'Drama'),
(3, 'Drama'),
(3, 'Romance'),
(4, 'Action'),
(4, 'Adventure'),
(5, 'Drama'),
(5, 'Crime');

-- --------------------------------------------------------

--
-- Structure de la table `realisateurs`
--

CREATE TABLE `realisateurs` (
  `film_id` int(11) DEFAULT NULL,
  `realisateur` varchar(255) DEFAULT NULL
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4;

--
-- Déchargement des données de la table `realisateurs`
--

INSERT INTO `realisateurs` (`film_id`, `realisateur`) VALUES
(1, 'Sam Raimi'),
(2, 'Francis Ford Coppola'),
(3, 'James Cameron'),
(4, 'George Lucas'),
(5, 'Frank Darabont');

--
-- Index pour les tables déchargées
--

--
-- Index pour la table `acteurs`
--
ALTER TABLE `acteurs`
  ADD PRIMARY KEY (`cast_actor_id`);

--
-- Index pour la table `films`
--
ALTER TABLE `films`
  ADD PRIMARY KEY (`film_id`),
  ADD UNIQUE KEY `slug_film_id` (`slug_film_id`);

--
-- Index pour la table `film_acteurs`
--
ALTER TABLE `film_acteurs`
  ADD KEY `film_id` (`film_id`),
  ADD KEY `cast_actor_id` (`cast_actor_id`);

--
-- Index pour la table `genres`
--
ALTER TABLE `genres`
  ADD KEY `film_id` (`film_id`);

--
-- Index pour la table `realisateurs`
--
ALTER TABLE `realisateurs`
  ADD KEY `film_id` (`film_id`);

--
-- AUTO_INCREMENT pour les tables déchargées
--

--
-- AUTO_INCREMENT pour la table `films`
--
ALTER TABLE `films`
  MODIFY `film_id` int(11) NOT NULL AUTO_INCREMENT, AUTO_INCREMENT=6;

--
-- Contraintes pour les tables déchargées
--

--
-- Contraintes pour la table `film_acteurs`
--
ALTER TABLE `film_acteurs`
  ADD CONSTRAINT `film_acteurs_ibfk_1` FOREIGN KEY (`film_id`) REFERENCES `films` (`film_id`),
  ADD CONSTRAINT `film_acteurs_ibfk_2` FOREIGN KEY (`cast_actor_id`) REFERENCES `acteurs` (`cast_actor_id`);

--
-- Contraintes pour la table `genres`
--
ALTER TABLE `genres`
  ADD CONSTRAINT `genres_ibfk_1` FOREIGN KEY (`film_id`) REFERENCES `films` (`film_id`);

--
-- Contraintes pour la table `realisateurs`
--
ALTER TABLE `realisateurs`
  ADD CONSTRAINT `realisateurs_ibfk_1` FOREIGN KEY (`film_id`) REFERENCES `films` (`film_id`);
COMMIT;

/*!40101 SET CHARACTER_SET_CLIENT=@OLD_CHARACTER_SET_CLIENT */;
/*!40101 SET CHARACTER_SET_RESULTS=@OLD_CHARACTER_SET_RESULTS */;
/*!40101 SET COLLATION_CONNECTION=@OLD_COLLATION_CONNECTION */;
