extern crate rand;
use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
	let mut reiniciar = true; //variable
	while reiniciar { //Ciclo para reiniciar el juego
    	let aleatorio = rand::thread_rng().gen_range(1,1001); //se toma un numero de un hilo de un rango
		//println!("El numero a adivinar es: {}", aleatorio); //En caso de querer saber que numero generado
		println!("====================== Adivina el Numero ======================");
		println!("Instrucciones: Se ha ganado un numero aleatorio en un rango del 1 al 1000, intenta adivinarlo, suerte!");
		let mut contador = 0; //contador que registra el numero de intentos realizados
	    let mut continuar = true; //Boolean para continuar el juego
    	while continuar {
	    	contador+= 1; // incrementamos el contador
	    	println!("Intento {}:", contador);
	    	let mut line = String::new(); //almacenador de lectura
	    	io::stdin().read_line(&mut line)
	    	.ok()
	    	.expect("No se pudo leer la linea"); //lee la linea y lanza excepcion si ocurre un error
	    	let line: u32 = line.trim().parse().ok().expect("Favor ingresar un numero");
	    	match line.cmp(&aleatorio) { //comparador de el numero secreto con el imput
				Ordering::Less => println!("Demasiado Bajo, intenta de nuevo"),
				Ordering::Greater => println!("Demasiado Alto, intenta de nuevo"),
				Ordering::Equal => println!("Adivinaste!!"),
			}//fin match
			if aleatorio == line { //comprobar que acerto en el resultado
				continuar = false; //salir de los intentos
			}//fin del if
	    }//fin while continuar
	    if contador < 10 { //contador para dar la clasificacion y el comentario acerca del numero de intentos
			println!("lo hizo en: {} intento(s),ya sabia ud el secreto, o tuvo suerte!.", contador);
		}else if contador == 10 {
			println!("Aja!, sabia ud el secreto, lo logro en {} intento(s).", contador);
		}else {
			println!("Deberia haberlo hecho mejor!, lo adivino en {} intento(s)", contador);
		}// fin if else
		let mut pregunta = true;
		while pregunta {
			println!("Desea volver a jugar? (s/n)"); //Pregunta si desea continuar
			let mut play_again = String::new(); //se declara una variable inmutable de tipo string
			io::stdin().read_line(&mut play_again)
			.ok()
			.expect("No se pudo leer la linea"); // lee respuesta a reiniciar
			if play_again.starts_with("s").to_string() == "true" || play_again.starts_with("S").to_string() == "true"{ //comprobar mediante string si elige si
				reiniciar = true; //salir del programa
				pregunta = false; //salir de pregunta
			}else if play_again.starts_with("n").to_string() == "true" || play_again.starts_with("N").to_string() == "true" { //comprobar mediante string si elige no
					reiniciar = false; // salir del programa
					pregunta = false; // para salir de pregunta cuando elija
			}else {
				println!("Elija una opcion"); // tiene que elegir una opcion a fuerza
			}// fin del if else
		}// fin del while de pregunta
    }//fin while reiniciar
    println!("Un puntito ingeniero!!!"); //No sea asi ingeniero, suelte ese puntito extra
}//fin main
