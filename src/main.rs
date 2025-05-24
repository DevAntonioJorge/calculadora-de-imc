use std::str::FromStr;


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
        let mut op = String::new();

        println!("Peso (kg): ");
        let peso = match read_input(&mut peso){
            Ok(num) => num,
            Err(error) => {
                println!("Erro ao digitar o peso: {}", error);
                continue;
            }
        };
        
        println!("Altura (m): ");
        let altura = match read_input(&mut altura){
            Ok(num) => num,
            Err(error) => {
                println!("Erro ao digitar a altura: {}", error);
                continue;
            }
        };

        
        let imc = calcular_imc(peso, altura);
        println!("Seu IMC é: {:.2}", imc);

        let classificacao = definir(imc);

        println!("Sua classificação de peso: {}", classificacao.classificar());
        
        println!("Deseja continuar? Aperte 1 para sair: ");

        let op: u8 = match read_input(&mut op){
            Ok(num) => num,
            Err(error) => {
                println!("Erro ao ler entrada: {}", error);
                continue;
            }
        };
        if op == 1 {
            println!("Volte sempre");
            break
        } else {
            continue;
        }
    }
}
fn read_input<T: FromStr>(input: &mut String) -> Result<T, String>
where
    <T as FromStr>::Err: std::fmt::Debug,
{
    input.clear();
    std::io::stdin()
        .read_line(input)
        .map_err(|e| format!("Erro ao ler entrada: {}", e))?;

    let value: T = input.trim().parse().map_err(|e| format!("Erro ao converter para número: {:?}", e))?;

    Ok(value)
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