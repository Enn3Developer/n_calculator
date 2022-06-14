use std::collections::HashMap;

use rug::ops::Pow;
use rug::Float;

type FnOp = Box<dyn Fn(Float, Float) -> Float + Sync>;

struct Op<F>
where
    F: Fn(Float, Float) -> Float,
{
    ops: HashMap<String, F>,
}

impl Op<FnOp> {
    fn new() -> Self {
        Self {
            ops: HashMap::new(),
        }
    }

    fn add(&mut self, key: String, op: FnOp) {
        self.ops.insert(key, op);
    }

    fn get(&self, key: &str) -> Option<&FnOp> {
        self.ops.get(key)
    }

    fn is_key_inside(&self, key: &str) -> bool {
        self.ops.contains_key(key)
    }
}

lazy_static! {
    static ref OPS: Op<FnOp> = {
        let mut ops = Op::new();

        ops.add(String::from("+"), Box::new(|a, b| a + b));
        ops.add(String::from("-"), Box::new(|a, b| a - b));
        ops.add(String::from("*"), Box::new(|a, b| a * b));
        ops.add(String::from("/"), Box::new(|a, b| a / b));
        ops.add(String::from("^"), Box::new(|a, b| a.pow(b)));

        ops
    };
}

fn is_numeric(string: &str) -> bool {
    string.parse::<f64>().is_ok()
}

pub fn replace_constants(expression: String) -> String {
    let mut expression = expression.replace("pi", "3.1415926535897932384626433832795028841971693993751058209749445923078164062862089986280348253421170679821480865132823066470938446095505822317253594081284811174502841027019385211055596446229489549303819644288109756659334461284756482337867831652712019091456485669234603486104543266482133936072602491412737245870066063155881748815209209628292540917153643678925903600113305305488204665213841469519415116094330572703657595919530921861173819326117931051185480744623799627495673518857527248912279381830119491298336733624406566430860213949463952247371907021798609437027705392171762931767523846748184676694051320005681271452635608277857713427577896091736371787214684409012249534301465495853710507922796892589235420199561121290219608640344181598136297747713099605187072113499999983729780499510597317328160963185950244594553469083026425223082533446850352619311881710100031378387528865875332083814206171776691473035982534904287554687311595628638823537875937519577818577805321712268066130019278766111959092164201989380952572010654858632788659361533818279682303019520353018529689957736225994138912497217752834791315155748572424541506959508295331168617278558890750983817546374649393192550604009277016711390098488240128583616035637076601047101819429555961989467678374494482553797747268471040475346462080466842590694912933136770289891521047521620569660240580381501935112533824300355876402474964732639141992726042699227967823547816360093417216412199245863150302861829745557067498385054945885869269956909272107975093029553211653449872027559602364806654991198818347977535663698074265425278625518184175746728909777727938000816470600161452491921732172147723501414419735685481613611573525521334757418494684385233239073941433345477624168625189835694855620992192221842725502542568876717904946016534668049886272327917860857843838279679766814541009538837863609506800642251252051173929848960841284886269456042419652850222106611863067442786220391949450471237137869609563643719172874677646575739624138908658326459958133904780275900994657640789512694683983525957098258226205224894077267194782684826014769909026401363944374553050682034962524517493996514314298091906592509372216964615157098583874105978859597729754989301617539284681382686838689427741559918559252459539594310499725246808459872736446958486538367362226260991246080512438843904512441365497627807977156914359977001296160894416948685558484063534220722258284886481584560285060168427394522674676788952521385225499546667278239864565961163548862305774564980355936345681743241125150760694794510965960940252288797108931456691368672287489405601015033086179286809208747609178249385890097149096759852613655497818931297848216829989487226588048575640142704775551323796414515237462343645428584447952658678210511413547357395231134271661021359695362314429524849371871101457654035902799344037420073105785390621983874478084784896833214457138687519435064302184531910484810053706146806749192781911979399520614196634287544406437451237181921799983910159195618146751426912397489409071864942319615679452080951465502252316038819301420937621378559566389377870830390697920773467221825625996615014215030680384477345492026054146659252014974428507325186660021324340881907104863317346496514539057");
    expression = expression.replace('e', "2.7182818284590452353602874713526624977572470936999595749669676277240766303535475945713821771660255131979437742518593242454861647180054967229205468117767817686998171518541462050721458131767108585802226");

    expression
}

pub fn check_expression(expression: &str) -> bool {
    let parts: Vec<&str> = expression.split(' ').collect();

    if parts.len() < 3 {
        return false;
    }

    let mut len_numbers = 0;
    let mut len_ops = 0;

    for part in parts {
        if is_numeric(part) {
            len_numbers += 1;
        } else {
            if len_numbers < 2 {
                return false;
            }

            if !OPS.is_key_inside(part) {
                return false;
            }

            len_ops += 1;
        }
    }

    len_numbers == len_ops + 1
}

pub fn evaluate_expression(expression: &str) -> Float {
    let mut numbers = vec![];
    let parts: Vec<&str> = expression.split(' ').collect();

    for part in parts {
        if is_numeric(part) {
            let mut part = part;
            let mut sign = false;
            if part.contains('-') {
                sign = true;
                part = &part[1..];
            }

            let num_parts: Vec<&str> = part.split('.').collect();
            let num = Float::with_val(200, num_parts[0].parse::<u128>().unwrap());
            let mut frac = 0.0;

            if let Some(decimal) = num_parts.get(1) {
                frac = (String::from("0.") + decimal).parse::<f64>().unwrap();
            }

            let mut number = num + frac;

            if sign {
                number *= -1;
            }

            numbers.push(number);
        } else {
            let b = numbers.pop().unwrap();
            let a = numbers.pop().unwrap();
            let result = (OPS.get(part).unwrap())(a, b);
            numbers.push(result);
        }
    }

    numbers[0].clone()
}

#[cfg(test)]
mod test {
    use crate::math::evaluate_expression;
    use rug::ops::Pow;
    use rug::Float;

    #[test]
    fn pow_test() {
        assert_eq!(
            Float::with_val(1, 2).pow(Float::with_val(1, 4)),
            Float::with_val(1, 16)
        )
    }

    #[test]
    fn negative_pow() {
        assert_eq!(
            Float::with_val(1, 2).pow(Float::with_val(1, -1)),
            Float::with_val(1, 0.5)
        )
    }

    #[test]
    fn fract_pow() {
        assert_eq!(
            Float::with_val(1, 4).pow(Float::with_val(1, 0.5)),
            Float::with_val(1, 2)
        )
    }

    #[test]
    fn expression() {
        assert_eq!(
            evaluate_expression("2 1 + 3 * 2 ^"),
            Float::with_val(10, 81)
        );
    }
}
