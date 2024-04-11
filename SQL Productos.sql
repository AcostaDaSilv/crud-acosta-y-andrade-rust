create table productos(
	nomprod varchar(250) not null,
	cantidad varchar(250) not null,
	productoid varchar(50) not null
	
);
create unique index productos_productoid_idx on productos(productoid)