use anchor_lang::prelude::*;

declare_id!("3VVwWtL67j6n7nc81vNUSGgvYVqiTT5HGXEcEzsFTegK");

#[program]
pub mod velocimetro {
    use super::*;

    // Crear velocimetro
    pub fn initialize(ctx: Context<Initialize>, max_speed: u64) -> Result<()> {
        let speedometer = &mut ctx.accounts.speedometer;

        speedometer.owner = ctx.accounts.user.key();
        speedometer.speed = 0;
        speedometer.max_speed = max_speed;

        Ok(())
    }

    // Acelerar
    pub fn accelerate(ctx: Context<UpdateSpeed>, amount: u64) -> Result<()> {
        let speedometer = &mut ctx.accounts.speedometer;

        let new_speed = speedometer.speed + amount;

        if new_speed > speedometer.max_speed {
            speedometer.speed = speedometer.max_speed;
        } else {
            speedometer.speed = new_speed;
        }

        Ok(())
    }

    // Frenar
    pub fn brake(ctx: Context<UpdateSpeed>, amount: u64) -> Result<()> {
        let speedometer = &mut ctx.accounts.speedometer;

        speedometer.speed = speedometer.speed.saturating_sub(amount);

        Ok(())
    }

    // Ver velocidad
    pub fn ver_velocidad(ctx: Context<VerVelocidad>) -> Result<()> {
        let speedometer = &ctx.accounts.speedometer;

        emit!(SpeedEvent {
            velocidad: speedometer.speed,
            max_velocidad: speedometer.max_speed,
        });

        Ok(())
    }

    // Actualizar velocidad maxima
    pub fn update_max_speed(ctx: Context<UpdateSpeed>, new_max: u64) -> Result<()> {
        let speedometer = &mut ctx.accounts.speedometer;

        speedometer.max_speed = new_max;

        if speedometer.speed > new_max {
            speedometer.speed = new_max;
        }

        Ok(())
    }

    // Eliminar velocimetro
    pub fn delete_speedometer(_ctx: Context<DeleteSpeedometer>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {

    #[account(
        init,
        payer = user,
        space = 8 + 32 + 8 + 8,
        seeds = [b"speed", user.key().as_ref()],
        bump
    )]
    pub speedometer: Account<'info, Speedometer>,

    #[account(mut)]
    pub user: Signer<'info>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct UpdateSpeed<'info> {

    #[account(
        mut,
        seeds = [b"speed", owner.key().as_ref()],
        bump,
        has_one = owner
    )]
    pub speedometer: Account<'info, Speedometer>,

    pub owner: Signer<'info>,
}

#[derive(Accounts)]
pub struct VerVelocidad<'info> {

    #[account(
        seeds = [b"speed", owner.key().as_ref()],
        bump
    )]
    pub speedometer: Account<'info, Speedometer>,

    pub owner: Signer<'info>,
}

#[derive(Accounts)]
pub struct DeleteSpeedometer<'info> {

    #[account(
        mut,
        close = owner,
        seeds = [b"speed", owner.key().as_ref()],
        bump,
        has_one = owner
    )]
    pub speedometer: Account<'info, Speedometer>,

    #[account(mut)]
    pub owner: Signer<'info>,
}

#[account]
pub struct Speedometer {
    pub owner: Pubkey,
    pub speed: u64,
    pub max_speed: u64,
}

#[event]
pub struct SpeedEvent {
    pub velocidad: u64,
    pub max_velocidad: u64,
}