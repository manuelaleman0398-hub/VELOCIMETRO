Explicación del Velocímetro en Solana con Anchor

Este programa está escrito en Rust usando el framework Anchor para crear un velocímetro on-chain en Solana. El contrato permite a un usuario crear un velocímetro, acelerar, frenar, consultar su velocidad, actualizar la velocidad máxima y eliminar el velocímetro.

A continuación se explica qué hace cada parte del código.

Importación de la librería principal

use anchor_lang::prelude::*;

Esta línea importa todas las herramientas principales del framework Anchor, como macros, tipos de cuentas, Result y estructuras necesarias para escribir programas en Solana.

ID del programa

declare_id!("3VVwWtL67j6n7nc81vNUSGgvYVqiTT5HGXEcEzsFTegK");

Aquí se declara el Program ID del contrato en Solana.
Este identificador es único y representa la dirección del programa desplegado en la blockchain.

Definición del programa

#[program]

Esta macro indica que el siguiente módulo contiene las instrucciones del programa que pueden ser llamadas desde clientes o frontends.

pub mod velocimetro {

Se define el módulo principal llamado velocimetro, donde estarán todas las funciones del contrato.

use super::*;

Importa todo lo definido fuera del módulo para poder usarlo dentro del programa.

Función Initialize (crear velocímetro)

pub fn initialize(ctx: Context<Initialize>, max_speed: u64) -> Result<()>

Esta función crea un nuevo velocímetro en la blockchain.

ctx contiene las cuentas necesarias para ejecutar la instrucción.

max_speed es la velocidad máxima que tendrá el velocímetro.

let speedometer = &mut ctx.accounts.speedometer;

Aquí se obtiene una referencia mutable a la cuenta speedometer para poder modificar sus datos.

speedometer.owner = ctx.accounts.user.key();

Se guarda la dirección pública del usuario como dueño del velocímetro.

speedometer.speed = 0;

Se inicializa la velocidad actual en 0.

speedometer.max_speed = max_speed;

Se guarda la velocidad máxima permitida.

Ok(())

Indica que la operación terminó correctamente.

Función Accelerate (acelerar)

pub fn accelerate(ctx: Context<UpdateSpeed>, amount: u64) -> Result<()>

Esta función aumenta la velocidad del velocímetro.

amount indica cuánto se incrementará la velocidad.

let speedometer = &mut ctx.accounts.speedometer;

Se obtiene acceso mutable al velocímetro.

let new_speed = speedometer.speed + amount;

Se calcula la nueva velocidad sumando la velocidad actual y el incremento.

if new_speed > speedometer.max_speed {

Se verifica si la nueva velocidad supera la velocidad máxima permitida.

speedometer.speed = speedometer.max_speed;

Si la supera, la velocidad se limita al valor máximo.

else { speedometer.speed = new_speed; }

Si no supera el máximo, se guarda la nueva velocidad.

Ok(())

Finaliza correctamente la función.

Función Brake (frenar)

pub fn brake(ctx: Context<UpdateSpeed>, amount: u64) -> Result<()>

Esta función reduce la velocidad del velocímetro.

let speedometer = &mut ctx.accounts.speedometer;

Se obtiene acceso mutable a la cuenta del velocímetro.

speedometer.speed = speedometer.speed.saturating_sub(amount);

Se reduce la velocidad usando saturating_sub, lo que evita que la velocidad se vuelva negativa.
Si el resultado sería negativo, simplemente se queda en 0.

Ok(())

La función termina correctamente.

Función Ver Velocidad

pub fn ver_velocidad(ctx: Context<VerVelocidad>) -> Result<()>

Esta función permite consultar la velocidad actual.

let speedometer = &ctx.accounts.speedometer;

Se obtiene acceso a la cuenta del velocímetro sin modificarla.

emit!(SpeedEvent { ... })

Se emite un evento en la blockchain con la velocidad actual y la velocidad máxima.

Esto permite que aplicaciones externas o frontends escuchen estos eventos.

Ok(())

Finaliza la ejecución.

Función Update Max Speed

pub fn update_max_speed(ctx: Context<UpdateSpeed>, new_max: u64) -> Result<()>

Esta función actualiza la velocidad máxima permitida.

let speedometer = &mut ctx.accounts.speedometer;

Se obtiene acceso mutable al velocímetro.

speedometer.max_speed = new_max;

Se guarda la nueva velocidad máxima.

if speedometer.speed > new_max

Se verifica si la velocidad actual supera el nuevo límite.

speedometer.speed = new_max;

Si es así, se ajusta la velocidad actual al nuevo máximo.

Ok(())

La función termina correctamente.

Función Delete Speedometer

pub fn delete_speedometer(_ctx: Context<DeleteSpeedometer>) -> Result<()>

Esta función elimina el velocímetro.

close = owner

La cuenta se cierra automáticamente y los lamports restantes se devuelven al propietario.

Ok(())

Confirma que la operación fue exitosa.

Estructuras de cuentas
Initialize

Define las cuentas necesarias para crear el velocímetro.

speedometer → cuenta donde se guardarán los datos

user → usuario que paga la creación

system_program → programa del sistema de Solana

space = 8 + 32 + 8 + 8

Define el tamaño de la cuenta:

8 bytes → discriminator de Anchor

32 bytes → Pubkey del dueño

8 bytes → velocidad

8 bytes → velocidad máxima

seeds = [b"speed", user.key().as_ref()]

Se usa un PDA (Program Derived Address) para generar una dirección única basada en el usuario.

UpdateSpeed

Define las cuentas necesarias para acelerar, frenar o actualizar velocidad.

mut

Permite modificar la cuenta.

has_one = owner

Garantiza que el dueño del velocímetro sea quien está firmando la transacción.

VerVelocidad

Permite consultar la velocidad sin modificar la cuenta.

DeleteSpeedometer

Define las cuentas necesarias para eliminar el velocímetro.

close = owner

Cierra la cuenta y envía el saldo al propietario.

Cuenta Speedometer

#[account]

Indica que esta estructura será almacenada en la blockchain.

pub struct Speedometer

Define los datos del velocímetro.

Campos:

owner → dueño del velocímetro

speed → velocidad actual

max_speed → velocidad máxima

Evento SpeedEvent

#[event]

Define un evento que puede ser emitido en la blockchain.

pub struct SpeedEvent

Contiene:

velocidad → velocidad actual

max_velocidad → velocidad máxima

Este evento se usa para notificar a aplicaciones externas cuando se consulta la velocidad.
