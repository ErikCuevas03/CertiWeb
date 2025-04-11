#![no_std] // No usamos la biblioteca estándar de Rust

use soroban_sdk::{contract, contractimpl, Env, String, Map, symbol_short};

/// Definimos la clave de almacenamiento como una constante para evitar errores tipográficos
const DOCUMENTS_KEY: soroban_sdk::Symbol = symbol_short!("DOCUMENTS");

#[contract]
pub struct DocumentosContract;

#[contractimpl]
impl DocumentosContract {
    // --- CREATE (Dar de alta un documento) ---
    /// Añade un nuevo documento al registro.
    /// Panics si el documento con el mismo ID ya existe.
    ///
    /// # Arguments
    ///
    /// * `id_documento` - El ID del documento (i32).
    /// * `titulo` - El título del documento (String).
    /// * `estado` - El estado del documento (String).
    /// * `fecha` - La fecha de creación del documento (u64).
    pub fn registrar_documento(env: Env, id_documento: i32, titulo: String, estado: String, fecha: u64) {
        // Obtenemos el mapa de documentos del almacenamiento persistente.
        // Si no existe, crea un mapa vacío.
        let mut documentos: Map<i32, (String, String, u64)> = env
            .storage()
            .persistent()
            .get(&DOCUMENTS_KEY)
            .unwrap_or(Map::new(&env)); // Crea un mapa vacío si no hay nada

        // Verificamos si el documento ya existe para evitar duplicados
        if documentos.contains_key(id_documento) {
            panic!("Documento con el mismo ID ya existe"); // Error si el documento ya está registrado
        }

        // Añadimos el nuevo documento con sus detalles (titulo, estado y fecha)
        documentos.set(id_documento, (titulo, estado, fecha));
        // Guardamos el mapa actualizado en el almacenamiento persistente
        env.storage().persistent().set(&DOCUMENTS_KEY, &documentos);
    }
    
    pub fn obtener_documento(env: Env, id_documento: i32) -> Option<(String, String, u64)> {
        let documentos: Map<i32, (String, String, u64)> = env
            .storage()
            .persistent()
            .get(&DOCUMENTS_KEY)
            .unwrap_or(Map::new(&env));
    
        documentos.get(id_documento)
    }
    
}
