
enum Classificacao{
    ObesidadeGrave,
    Obesidade,
    Sobrepeso,
    PesoNormal,
    PesoAbaixo,
    Invalido
}

impl Classificacao {
    fn classificar(self) -> String{
        match self{
            Classificacao::ObesidadeGrave => "Obesidade Grave".to_string(),
            Classificacao::Obesidade => "Obesidade".to_string(),
            Classificacao::Sobrepeso => "Obesidade Grave".to_string(),
            Classificacao::PesoNormal => "Peso normal".to_string(),
            Classificacao::PesoAbaixo => "Abaixo do peso".to_string(),
            Classificacao::Invalido => "Imc inválido".to_string()
        }
    }
}

fn main() {
    println!("Bem vindo à calculadora de imc!");
    println!("Para calcular o seu IMC, insira seu peso em kg e sua altura em metros.");
    loop {
        let mut peso = String::new();
        let mut altura = String::new();

        println!("Peso (kg): ");
        std::io::stdin()
            .read_line(&mut peso)
            .unwrap();

        
        println!("Altura (m): ");
        std::io::stdin()
            .read_line(&mut altura)
            .unwrap();
        let peso: f32 = peso.trim().parse().expect("Peso inválido");
        let altura: f32 = altura.trim().parse().expect("Altura inválida");
        
        let imc = calcular_imc(peso, altura);
        println!("Seu IMC é: {:.2}", imc);

        let classificacao = definir(imc);

        println!("Sua classificação de peso: {}", classificacao.classificar());
        break
    }
}

fn calcular_imc(peso: f32, altura: f32) -> f32{
    let imc = peso / (altura * altura);
    imc
}

fn definir(imc: f32) -> Classificacao{
    if imc >= 40.0{
        Classificacao::ObesidadeGrave
    } else if imc > 30.0 && imc < 39.9{
        Classificacao::Obesidade
    } else if imc > 25.0 && imc < 29.9{
        Classificacao::Sobrepeso
    } else if imc > 18.5 && imc < 24.9{
        Classificacao::PesoNormal
    } else if imc < 18.5{
        Classificacao::PesoAbaixo
    } else {
        Classificacao::Invalido
    }
}