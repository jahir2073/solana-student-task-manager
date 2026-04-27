use anchor_lang::prelude::*;

declare_id!("11111111111111111111111111111111");

#[program]
pub mod student_task_manager {
    use super::*;

    // Crear una nueva tarea
    pub fn create_task(
        ctx: Context<CreateTask>,
        title: String,
        description: String,
    ) -> Result<()> {
        let task = &mut ctx.accounts.task;

        task.owner = *ctx.accounts.user.key;
        task.title = title;
        task.description = description;
        task.completed = false;

        Ok(())
    }

    // Editar descripción de la tarea
    pub fn update_task(
        ctx: Context<UpdateTask>,
        new_description: String,
    ) -> Result<()> {
        let task = &mut ctx.accounts.task;
        task.description = new_description;

        Ok(())
    }

    // Marcar tarea como completada
    pub fn complete_task(
        ctx: Context<CompleteTask>,
    ) -> Result<()> {
        let task = &mut ctx.accounts.task;
        task.completed = true;

        Ok(())
    }

    // Eliminar tarea
    pub fn delete_task(
        _ctx: Context<DeleteTask>,
    ) -> Result<()> {
        Ok(())
    }
}

// Estructura donde se guarda cada tarea
#[account]
pub struct Task {
    pub owner: Pubkey,
    pub title: String,
    pub description: String,
    pub completed: bool,
}

// Contexto para crear tarea
#[derive(Accounts)]
#[instruction(title:String)]
pub struct CreateTask<'info> {

    #[account(
        init,
        payer = user,
        space = 8 + 32 + 100 + 300 + 1,
        seeds = [b"task", user.key().as_ref(), title.as_bytes()],
        bump
    )]
    pub task: Account<'info, Task>,

    #[account(mut)]
    pub user: Signer<'info>,

    pub system_program: Program<'info, System>,
}

// Contexto para actualizar tarea
#[derive(Accounts)]
pub struct UpdateTask<'info> {

    #[account(mut)]
    pub task: Account<'info, Task>,

    pub owner: Signer<'info>,
}

// Contexto para completar tarea
#[derive(Accounts)]
pub struct CompleteTask<'info> {

    #[account(mut)]
    pub task: Account<'info, Task>,

    pub owner: Signer<'info>,
}

// Contexto para eliminar tarea
#[derive(Accounts)]
pub struct DeleteTask<'info> {

    #[account(mut, close = owner)]
    pub task: Account<'info, Task>,

    #[account(mut)]
    pub owner: Signer<'info>,
}
