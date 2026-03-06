Tienda de Videojuegos en Solana (Anchor)

Este proyecto es un smart contract desarrollado con Anchor en Solana que simula una tienda de videojuegos on-chain.

Permite realizar operaciones básicas de gestión de inventario utilizando el patrón CRUD:

Crear tienda

Agregar videojuegos

Actualizar videojuegos

Eliminar videojuegos

Ver inventario

El contrato almacena la información en una cuenta PDA que pertenece al dueño de la tienda.

Tecnologías utilizadas

Solana Blockchain

Rust

Anchor Framework

Program Derived Addresses (PDA)

Estructura del Programa

El programa se divide en:

Programa principal
#[program]
pub mod tienda_videojuegos


Contiene todas las instrucciones que interactúan con la cuenta de la tienda.

Funcionalidades
Crear tienda

Inicializa una cuenta PDA donde se guardará el inventario.

crear_tienda(nombre)


Parámetros:

nombre → Nombre de la tienda

Datos almacenados:

Nombre de la tienda

Owner

Inventario de videojuegos

Agregar videojuego

Agrega un nuevo videojuego al inventario.

agregar_videojuego(titulo, precio, stock)


Parámetros:

titulo → Nombre del videojuego

precio → Precio del juego

stock → Cantidad disponible

Restricción:

Máximo 20 videojuegos en el inventario.

Actualizar videojuego

Permite modificar precio o stock de un videojuego existente.

actualizar_juego(titulo, nuevo_precio, nuevo_stock)


Parámetros:

titulo

nuevo_precio (opcional)

nuevo_stock (opcional)

Eliminar videojuego

Elimina un videojuego del inventario.

eliminar_videojuego(titulo)


Busca el videojuego por título y lo elimina del vector.

Ver inventario

Muestra el inventario completo de la tienda en los logs.

ver_inventario()


Salida en consola:

--- TIENDA: GameStore ---
Juego: Halo | Precio: 60 | Stock: 10
Juego: Zelda | Precio: 70 | Stock: 5

Estructuras de datos
TiendaVideojuegos
pub struct TiendaVideojuegos {
    pub nombre: String,
    pub owner: Pubkey,
    pub inventario: Vec<Videojuego>,
}


Contiene:

Nombre de la tienda

Dueño de la tienda

Lista de videojuegos

Videojuego
pub struct Videojuego {
    pub titulo: String,
    pub precio: u64,
    pub stock: u32,
}


Contiene:

Título

Precio

Stock disponible

Seguridad

El contrato incluye verificaciones importantes.

Verificación de propietario
#[account(mut, has_one = owner)]


Solo el dueño de la tienda puede:

Agregar juegos

Actualizar juegos

Eliminar juegos

Límite de inventario
require!(tienda.inventario.len() < 20)


Se limita a 20 juegos para evitar exceder el límite de 10KB de las cuentas en Solana.

Manejo de errores

Errores definidos:

ErrorCode

JuegoNoEncontrado

Se lanza cuando el videojuego no existe en el inventario.

El videojuego no existe en la tienda.

TiendaLlena

Se lanza cuando se intenta agregar más de 20 juegos.

La tienda ya no tiene espacio para más juegos.

PDA utilizada

La cuenta de la tienda se genera con:

seeds = ["tienda_vj", owner]


Esto garantiza que cada usuario tenga su propia tienda única.

Ejemplo de flujo

Crear tienda

crear_tienda("GameStore")


Agregar juegos

agregar_videojuego("Halo", 60, 10)
agregar_videojuego("Zelda", 70, 5)


Actualizar juego

actualizar_juego("Halo", Some(65), None)


Ver inventario

ver_inventario()


Eliminar juego

eliminar_videojuego("Zelda")
