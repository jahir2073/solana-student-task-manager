# Solana Student Task Manager

## Descripción

Proyecto desarrollado en Solana usando Rust + Anchor.

Es una aplicación descentralizada para administrar tareas escolares o personales dentro de blockchain.

Cada usuario puede:

- Crear tareas
- Editar tareas
- Marcar tareas completadas
- Eliminar tareas

## Tecnologías utilizadas

- Solana Blockchain
- Rust
- Anchor Framework

## Requisitos implementados

Proyecto individual  
Repositorio público en GitHub  
Proyecto libre  
Desarrollado en Solana (Rust + Anchor)  
CRUD  
PDA  
Documentación del proyecto  

## CRUD

- Create → Crear tarea
- Read → Consultar tarea
- Update → Editar tarea
- Delete → Eliminar tarea

## PDA

Cada tarea utiliza una cuenta única generada con:

["task", wallet_usuario, titulo]

## Estructura del proyecto

programs/student-task-manager/src/lib.rs

## Autor

Jahir Emmanuel
