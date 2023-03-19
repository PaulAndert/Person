
CREATE SCHEMA `person`;

use person;

drop table children; drop table person; drop table family;

create table person(
	person_id int auto_increment,
	first_name varchar(255),
    middle_name varchar(255),
    surname varchar(255),
    maiden_name varchar(255),
    gender varchar(1),
    birthday date,
    deathday date,
	constraint person_pk primary key (person_id)
);

create table family (
    family_id int auto_increment,
	male_id int,
    female_id int,
    constraint family_pk primary key (family_id)
);

create table children (
    person_id int NOT NULL,
    family_id int NOT NULL,
    constraint person_family_pk primary key (person_id, family_id)
);

select p.* from person p join children c on p.person_id = c.person_id where p.person_id = 1;

select f.* from family f join children c on f.family_id = c.family_id where male_id = {} or female_id = {} or c.person_id = {};

select p.* from family f 
join children c on f.family_id = c.family_id 
join person p on p.person_id = c.person_id 
where f.male_id = {} and f.female_id = {};


select male_id from family f 
join children c on f.family_id = c.family_id 
join person p on p.person_id = c.person_id 
where c.person_id = 1;

select female_id from family f 
join children c on f.family_id = c.family_id 
join person p on p.person_id = c.person_id 
where c.person_id = 1;

select * from person where person_id = any(
select male_id from family f 
join children c on f.family_id = c.family_id 
join person p on p.person_id = c.person_id 
where c.person_id = 1) or person_id = any(
select female_id from family f 
join children c on f.family_id = c.family_id 
join person p on p.person_id = c.person_id 
where c.person_id = 1
);


select p.* from person p join children c on p.person_id = c.person_id where p.person_id = 6;


 select p.* from person p 
 join children c on p.person_id = c.person_id 
 join family f on c.family_id = f.family_id
 where f.family_id = 
 (select family_id from children where person_id = 18);