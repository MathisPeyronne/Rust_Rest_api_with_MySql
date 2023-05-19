-- phpMyAdmin SQL Dump
-- version 4.9.0.1
-- https://www.phpmyadmin.net/
--
-- Host: localhost:8889
-- Generation Time: Apr 18, 2023 at 05:39 PM
-- Server version: 5.7.26
-- PHP Version: 7.3.8

CREATE DATABASE Projet_BD_Film CHARACTER SET utf8mb4 COLLATE utf8mb4_unicode_ci;
use Projet_BD_Film;

SET SQL_MODE = "NO_AUTO_VALUE_ON_ZERO";
SET time_zone = "+00:00";

--
-- Database: `escalade`
--

-- --------------------------------------------------------

--
-- Table structure for table `CATEGORIES`
--

CREATE TABLE `student` (
  `name` varchar(6) DEFAULT NULL,
  `email` varchar(1) DEFAULT NULL,
  `age` int(5) DEFAULT NULL,
) ENGINE=InnoDB DEFAULT CHARSET=utf8;

--
-- Dumping data for table `CATEGORIES`
--

INSERT INTO `student` (`name`, `email`, `age`) VALUES
('Leonardo DiCaprio', 'fds@gmail.com', 21),
('Ldofdsfs DiCaprio', 'fds@gfsdmail.com', 24)
;
-- --------------------------------------------------------

--
-- Table structure for table `RESULTATS`
--



--
-- Dumping data for table `RESULTATS`
--


