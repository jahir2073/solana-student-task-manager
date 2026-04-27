# Solana Student Task Manager

## Descripción del proyecto

Desarrollé una aplicación descentralizada llamada **Solana Student Task Manager**, enfocada en la organización de tareas escolares y personales dentro de la blockchain de Solana.

La idea principal del proyecto fue crear una herramienta sencilla donde un estudiante pueda administrar sus pendientes de forma segura, utilizando tecnología blockchain y contratos inteligentes.

Con este sistema, cada usuario puede guardar sus propias tareas, modificarlas, marcarlas como completadas o eliminarlas.

Ejemplos de tareas:

* Entregar proyecto final
* Estudiar cálculo integral
* Hacer práctica de bases de datos
* Preparar exposición

---

# Objetivo del proyecto

El objetivo fue aplicar los conocimientos aprendidos durante la certificación, desarrollando un programa real en Solana usando:

* Rust
* Anchor Framework
* CRUD
* PDA
* Smart Contracts

Además, busqué desarrollar un proyecto útil, práctico y fácil de entender.

---

# Tecnologías utilizadas

* Solana Blockchain
* Rust
* Anchor Framework
* GitHub

---

# ¿Qué hace mi proyecto?

Mi programa permite administrar tareas mediante cuatro funciones principales:

## 1. Crear tarea

El usuario puede registrar una nueva tarea agregando:

* título
* descripción

Ejemplo:

Título: Proyecto Final
Descripción: Terminar documentación y subir repositorio.

---

## 2. Editar tarea

Permite modificar la descripción de una tarea existente.

Ejemplo:

Antes: Estudiar matemáticas
Después: Estudiar matemáticas unidad 3.

---

## 3. Completar tarea

Permite marcar una tarea como terminada.

Cuando esto sucede, el estado cambia a verdadero.

---

## 4. Eliminar tarea

Permite borrar una tarea del sistema y cerrar la cuenta correspondiente.

---

# ¿Cómo funciona?

Cuando un usuario crea una tarea:

1. Firma la transacción con su wallet.
2. El programa genera una cuenta única.
3. La información queda almacenada en Solana.
4. Después puede editarla, completarla o eliminarla.

Cada tarea pertenece únicamente al usuario que la creó.

---

# Estructura principal de datos

Utilicé una estructura llamada `Task` para almacenar la información:

```rust
pub struct Task {
    pub owner: Pubkey,
    pub title: String,
    pub description: String,
    pub completed: bool,
}
```

## Explicación

* owner → dueño de la tarea
* title → nombre de la tarea
* description → detalles
* completed → estado de la tarea

---

# Código importante utilizado

## Importación principal

```rust
use anchor_lang::prelude::*;
```

Esta librería permite trabajar con Anchor y Solana.

---

## Program ID

```rust
declare_id!("11111111111111111111111111111111");
```

Identifica el contrato inteligente dentro de la red.

---

## Declaración del programa

```rust
#[program]
```

Aquí se definen las funciones principales del sistema.

---

# CRUD implementado

## Create

```rust
create_task()
```

Crea una nueva tarea.

## Read

La información almacenada puede consultarse en la cuenta creada.

## Update

```rust
update_task()
complete_task()
```

Actualiza descripción o estado.

## Delete

```rust
delete_task()
```

Elimina una tarea.

---

# PDA implementado

Para organizar las cuentas utilicé PDA (Program Derived Address).

Código utilizado:

```rust
seeds = [b"task", user.key().as_ref(), title.as_bytes()]
```

Esto genera una dirección única usando:

* palabra task
* wallet del usuario
* título de la tarea

Gracias a esto cada tarea tiene su propia cuenta.

---

# Seguridad del proyecto

Utilicé firmas del usuario para validar operaciones.

```rust
pub owner: Signer<'info>
```

Esto ayuda a que solo el propietario pueda modificar sus tareas.

---

# Estructura del repositorio

```text
solana-student-task-manager/
│── README.md
│── Anchor.toml
│── Cargo.toml
└── programs/
    └── student-task-manager/
        └── src/
            └── lib.rs
```

---

# Archivos importantes

## README.md

Incluye la documentación general del proyecto.

## Cargo.toml

Configuración de Rust.

## Anchor.toml

Configuración del framework Anchor.

## lib.rs

Archivo principal donde programé el contrato inteligente.

---

# Cómo usar el proyecto

1. Crear una tarea nueva
2. Consultar la información
3. Editar la tarea
4. Marcarla completada
5. Eliminarla

---

# Lo que aprendí con este proyecto

Durante el desarrollo reforcé conocimientos sobre:

* Smart contracts
* Estructuras en Rust
* Cuentas en Solana
* PDA
* Organización de proyectos blockchain

---

# Mejoras futuras

En versiones futuras me gustaría agregar:

* Fechas límite
* Prioridades
* Interfaz web
* Categorías
* Historial de tareas

---

# Conclusión

Solana Student Task Manager fue desarrollado como una solución práctica para organizar tareas estudiantiles usando blockchain. El proyecto me permitió aplicar conceptos importantes como Rust, Anchor, CRUD y PDA, cumpliendo correctamente los requisitos solicitados en la certificación.

---

# Autor

Jahir Emmanuel sanchez marin 
