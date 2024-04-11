#![allow(dead_code)]
use std::error::Error;
use sqlx::FromRow;
use std::io;

#[derive(Debug, FromRow)]
struct Producto {
    pub nomprod: String,
    pub cantidad: String,
    pub prodid: String,
}

async fn create (producto: &Producto, pool: &sqlx::PgPool) -> Result<(), Box<dyn Error>> {
    let query = "INSERT INTO productos (nomprod, cantidad, prodid) VALUES ($1, $2, $3)";

    sqlx::query(query)
        .bind(&producto.nomprod)
        .bind(&producto.cantidad)
        .bind(&producto.prodid)
        .execute(pool)
        .await?;
    
    Ok(())
}

async fn update(producto: &Producto, prodid: &str, pool: &sqlx::PgPool) -> Result<(), Box<dyn Error>> {
    let query = "UPDATE producto SET nomprod = $1, cantidad = $2 WHERE prodid = $3";

    sqlx::query(query)
        .bind(&producto.nomprod)
        .bind(&producto.cantidad)
        .bind(&producto.prodid)
        .execute(pool)
        .await?;

    Ok(())
}

async fn read(conn: &sqlx::PgPool) -> Result<Vec<Producto>, Box<dyn Error>> {
    let q = "SELECT nomprod, cantidad, prodid FROM productos";
    
    let query = sqlx::query_as::<_, Producto>(q);
    
    let productos = query.fetch_all(conn).await?;
    
    for producto in &productos{
        println!("___________________________________________________");
        println!(": {}", producto.nomprod);
        println!("Cantidad: {}", producto.cantidad);
        println!("producto ID: {}", producto.prodid);
    };

    Ok(productos)
}

async fn delete(productoid: &str, pool: &sqlx::PgPool) -> Result<(), Box<dyn Error>> {
    let q = format!("DELETE FROM productos WHERE productoid = '{}'", productoid);

    let query = sqlx::query(&q).execute(pool).await?;
    println!("producto eliminado exitosamente");
    
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let url = "postgres://username:password@localhost:5432/productos";
    let pool = sqlx::postgres::PgPool::connect(url).await?;

    let mut opcion = String::new();

    loop {
        println!("Menú de opciones:");
        println!("1. Añadir producto");
        println!("2. Editar producto");
        println!("3. Leer productos");
        println!("4. Eliminar productos");
        println!("5. Salir");

        println!("Ingrese su opción: ");
        io::stdin().read_line(&mut opcion).unwrap();

        match opcion.trim() {
        "1" => {
            println!("Ingrese el nombre del producto:");
            let mut nom = String::new();
            io::stdin().read_line(&mut nom).unwrap();
            
            println!("Ingrese la cantidad del producto:");
            let mut cant = String::new();
            io::stdin().read_line(&mut cant).unwrap();
            
            println!("Ingrese el Id del producto:");
            let mut identificador = String::new();
            io::stdin().read_line(&mut identificador).unwrap();

            let producto = Producto{
                nomprod: nom.to_string(),
                cantidad: cant.to_string(),
                prodid: identificador.to_string(),
            };
            create(&producto, &pool).await?;

            println!("Añadiendo producto...");
        }
        "2" => {
            println!("Ingrese el Id del producto:");
            let mut identificador = String::new();
            io::stdin().read_line(&mut identificador).unwrap();

            println!("Ingrese el nuevo nombre del producto:");
            let mut nom = String::new();
            io::stdin().read_line(&mut nom).unwrap();
            
            println!("Ingrese la nueva cantidad del producto:");
            let mut cant = String::new();
            io::stdin().read_line(&mut cant).unwrap();

            let producto = Producto{
                nomprod: nom.to_string(),
                cantidad: cant.to_string(),
                prodid: identificador.to_string(),
            };
            update(&producto, &producto.prodid, &pool).await?;
            
            println!("Modificando producto...");
        }
        "3" => {
            let mut espera = String::new();
            println!("Leyendo productos...");
            read(&pool).await?;
            println!("_______________________________________________");
            println!("Presione enter para continuar");
            io::stdin().read_line(&mut espera).unwrap();
        }
        "4" => {
            let mut espera = String::new();
            println!("Ingrese el Id del producto que desea eliminar:");
            let mut identificador = String::new();
            io::stdin().read_line(&mut identificador).unwrap();

            delete(&identificador, &pool).await?;
            println!("Eliminando producto...");
            println!("_______________________________________________");
            println!("Presione enter para continuar");
            io::stdin().read_line(&mut espera).unwrap();
        }
        "5" => {
            println!("Saliendo del menú...");
            break;
        }
        _ => {
            println!("Opción inválida. Intente nuevamente.");
        }
        }

        opcion.clear();
    }

    Ok(())
}
