
CREATE SCHEMA `person`;

use person;

create table person(
	person_id int auto_increment,
	first_name varchar(255),
    middle_name varchar(255),
    surname varchar(255),
    maiden_name varchar(255),
    gender varchar(1),
    birthday varchar(255),
    deathday varchar(255),
	constraint person_pk primary key (person_id)
);

create table relation (
	male_id int,
    female_id int,
    child_id int
);

