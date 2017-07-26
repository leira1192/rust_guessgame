extern crate rand;
use std::io;
use rand::Rng;
use std::cmp::Ordering;


fn main() {
	let reiniciar = true;
	while reiniciar {
    	let aleatorio = rand::thread_rng().gen_range(1,1001);
		println!("El numero a adivinar es: {}", aleatorio);
		println!("Adivina el Numero");
		println!("Instrucciones: Adivina el numero en un rango del 1 al 1000, suerte!");
		let mut contador = 0; //contador que registra el numero de intentos realizados
	    let mut continuar = true; //Boolean para continuar el juego
    	while continuar {
	    	contador+= 1; // incrementamos el contador
	    	println!("Intento {}:", contador);
	    	let mut line = String::new(); //almacenador de lectura
	    	io::stdin().read_line(&mut line)
	    	.ok()
	    	.expect("No se pudo leer la linea"); //Read
	    	let line: u32 = line.trim().parse().ok().expect("Favor ingresar un numero");
	    	match line.cmp(&aleatorio) { //comparador de el numero secreto con el imput
				Ordering::Less => println!("Demasiado Bajo, intenta de nuevo"),
				Ordering::Greater => println!("Demasiado Alto, intenta de nuevo"),
				Ordering::Equal => println!("Adivinaste!!"),
			}//fin match
			if aleatorio == line {
				println!("Presione cualquier tecla para reiniciar, (n) para salir?");
				let mut play_again = String::new();
				io::stdin().read_line(&mut play_again)
				.ok()
				.expect("No se pudo leer la linea"); // Read continuous
				continuar = false;
				/*
					if assert_eq!(play_again, "n" ) {
					reiniciar = false;
				}//fin del if
				*/
			}//fin del if
	    }//fin while continuar
    }//fin while reiniciar
}//fin main
