extern crate rand;
use std::io;
use rand::Rng;
use std::cmp::Ordering;
//use std::string;


fn main() {
	let mut reiniciar = true;
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
				continuar = false;
			}//fin del if
	    }//fin while continuar
	    if contador < 10 {
			println!("Ya sabia ud el secreto, o tuvo suerte!");
		}else if contador == 10 {
			println!("Aja!, sabia ud el secreto");
		}else {
			println!("Deberia haberlo hecho mejor!");
		}// fin if else
		let mut pregunta = true;
		while pregunta {
			println!("Desea volver a jugar? (s/n)");
			let mut play_again = String::new();
			io::stdin().read_line(&mut play_again)
			.ok()
			.expect("No se pudo leer la linea"); // Read continuous
			if play_again.starts_with("s").to_string() == "true" || play_again.starts_with("S").to_string() == "true"{
				reiniciar = true;
				pregunta = false;
			}else if play_again.starts_with("n").to_string() == "true" || play_again.starts_with("N").to_string() == "true" {
					reiniciar = false;
					pregunta = false;
			}else {
				println!("Elija una opcion");
			}	
		}
    }//fin while reiniciar
}//fin main
