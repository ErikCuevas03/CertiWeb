#![no_std] // No usamos la biblioteca estándar de Rust

use soroban_sdk::{contract, contractimpl, Env, String, Map, symbol_short};

/// Definimos la clave de almacenamiento como una constante para evitar errores tipográficos
const DOCUMENTS_KEY: soroban_sdk::Symbol = symbol_short!("DOCUMENTS");
const HISTORIAL_KEY: soroban_sdk::Symbol = symbol_short!("HISTORIAL");
const RESPALDOS_KEY: soroban_sdk::Symbol = symbol_short!("RESPALDOS");


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
 //metodo para obtener todos los documentos   
    pub fn obtener_documento(env: Env, id_documento: i32) -> Option<(String, String, u64)> {
        let documentos: Map<i32, (String, String, u64)> = env
            .storage()
            .persistent()
            .get(&DOCUMENTS_KEY)
            .unwrap_or(Map::new(&env));
    
        documentos.get(id_documento)
    }
//metodo para realizar una consulta al historial   /// Añade una entrada al historial de un documento.
    pub fn consulta_historial(env: Env, id_historial: i32, fecha: u64, resultado: String) {
        let mut historial: Map<i32, (u64, String)> = env
            .storage()
            .persistent()
            .get(&HISTORIAL_KEY)
            .unwrap_or(Map::new(&env));

        if historial.contains_key(id_historial) {
            panic!("Historial con el mismo ID ya existe");
        }

        historial.set(id_historial, (fecha, resultado));
        env.storage().persistent().set(&HISTORIAL_KEY, &historial);
    }
    //  --- Obtener una entrada del historial por ID ---
    pub fn obtener_historial(env: Env, id_historial: i32) -> Option<(u64, String)> {
        let historial: Map<i32, (u64, String)> = env
            .storage()
            .persistent()
            .get(&HISTORIAL_KEY)
            .unwrap_or(Map::new(&env));

        historial.get(id_historial)
    }
// --- Realizar un respaldo de un documento --- 
    /// Añade un respaldo de un documento al registro.
    pub fn realizar_respaldo(
        env: Env,
        id_respaldo: i32,
        fecha: u64,
        ubicacion: String,
        autor: String,
    ) {
        let mut respaldos: Map<i32, (u64, String, String)> = env
            .storage()
            .persistent()
            .get(&RESPALDOS_KEY)
            .unwrap_or(Map::new(&env));
    
        if respaldos.contains_key(id_respaldo) {
            panic!("Respaldo con ese ID ya existe");
        }  
        respaldos.set(id_respaldo, (fecha, ubicacion, autor));
        env.storage().persistent().set(&RESPALDOS_KEY, &respaldos);
    }
 // --- Obtener un respaldo por ID ---
    /// Devuelve un respaldo de un documento por su ID.
    /// Devuelve `None` si no existe.   
    pub fn consultar_respaldo(env: Env, id_respaldo: i32) -> Option<(u64, String, String)> {
        let respaldos: Map<i32, (u64, String, String)> = env
            .storage()
            .persistent()
            .get(&RESPALDOS_KEY)
            .unwrap_or(Map::new(&env));
    
        respaldos.get(id_respaldo)
    }
    
}
