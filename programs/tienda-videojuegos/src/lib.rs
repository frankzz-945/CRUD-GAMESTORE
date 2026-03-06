use anchor_lang::prelude::*;

// ID del programa (Asegúrate de que coincida con tu Anchor.toml)
declare_id!("Dff5us8pSptXmR8GvWHrQkudUDY89rS2L5TRoygDYZY9");

#[program]
pub mod tienda_videojuegos {
    use super::*;

    // 1. INICIALIZAR TIENDA
    pub fn crear_tienda(ctx: Context<CrearTienda>, nombre: String) -> Result<()> {
        let tienda = &mut ctx.accounts.tienda;
        tienda.nombre = nombre;
        tienda.owner = ctx.accounts.owner.key();
        tienda.inventario = Vec::new();
        msg!("Tienda '{}' creada exitosamente", tienda.nombre);
        Ok(())
    }

    // 2. AGREGAR (CREATE)
    pub fn agregar_videojuego(
        ctx: Context<GestionTienda>,
        titulo: String,
        precio: u64,
        stock: u32,
    ) -> Result<()> {
        let tienda = &mut ctx.accounts.tienda;
        
        // Verificamos si hay espacio en el Vec (limite definido en InitSpace)
        require!(tienda.inventario.len() < 20, ErrorCode::TiendaLlena);

        let nuevo_juego = Videojuego {
            titulo,
            precio,
            stock,
        };

        tienda.inventario.push(nuevo_juego);
        msg!("Videojuego agregado: {}", tienda.inventario.last().unwrap().titulo);
        Ok(())
    }

    // 3. ACTUALIZAR (UPDATE)
    pub fn actualizar_juego(
        ctx: Context<GestionTienda>,
        titulo: String,
        nuevo_precio: Option<u64>,
        nuevo_stock: Option<u32>,
    ) -> Result<()> {
        let tienda = &mut ctx.accounts.tienda;
        
        let juego = tienda.inventario.iter_mut()
            .find(|j| j.titulo == titulo)
            .ok_or(ErrorCode::JuegoNoEncontrado)?;

        if let Some(p) = nuevo_precio { juego.precio = p; }
        if let Some(s) = nuevo_stock { juego.stock = s; }

        msg!("Juego '{}' actualizado.", titulo);
        Ok(())
    }

    // 4. ELIMINAR (DELETE)
    pub fn eliminar_videojuego(ctx: Context<GestionTienda>, titulo: String) -> Result<()> {
        let tienda = &mut ctx.accounts.tienda;

        let pos = tienda.inventario.iter()
            .position(|j| j.titulo == titulo)
            .ok_or(ErrorCode::JuegoNoEncontrado)?;

        tienda.inventario.swap_remove(pos);
        msg!("Juego '{}' eliminado del inventario.", titulo);
        Ok(())
    }

    // 5. VER FEED (READ/LOG)
    pub fn ver_inventario(ctx: Context<VerTienda>) -> Result<()> {
        let tienda = &ctx.accounts.tienda;
        msg!("--- TIENDA: {} ---", tienda.nombre);
        
        for juego in tienda.inventario.iter() {
            msg!("Juego: {} | Precio: {} | Stock: {}", 
                juego.titulo, juego.precio, juego.stock);
        }
        Ok(())
    }
}

// --- ESTRUCTURAS DE DATOS ---

#[account]
#[derive(InitSpace)]
pub struct TiendaVideojuegos {
    #[max_len(50)]
    pub nombre: String,
    pub owner: Pubkey,
    #[max_len(20)] // Límite de 20 juegos en el feed para no exceder los 10KB de una PDA simple
    pub inventario: Vec<Videojuego>,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, InitSpace)]
pub struct Videojuego {
    #[max_len(50)]
    pub titulo: String,
    pub precio: u64,
    pub stock: u32,
}

// --- CONTEXTOS DE INSTRUCCIONES ---

#[derive(Accounts)]
pub struct CrearTienda<'info> {
    #[account(
        init,
        payer = owner,
        space = 8 + TiendaVideojuegos::INIT_SPACE,
        seeds = [b"tienda_vj", owner.key().as_ref()],
        bump
    )]
    pub tienda: Account<'info, TiendaVideojuegos>,
    #[account(mut)]
    pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct GestionTienda<'info> {
    #[account(mut, has_one = owner)]
    pub tienda: Account<'info, TiendaVideojuegos>,
    pub owner: Signer<'info>,
}

#[derive(Accounts)]
pub struct VerTienda<'info> {
    pub tienda: Account<'info, TiendaVideojuegos>,
}

// --- ERRORES ---

#[error_code]
pub enum ErrorCode {
    #[msg("El videojuego no existe en la tienda.")]
    JuegoNoEncontrado,
    #[msg("La tienda ya no tiene espacio para más juegos.")]
    TiendaLlena,
}