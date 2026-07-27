const GRENZE: usize = 1_000_000;
const FAKULTAETEN: [usize; 10] = [1, 1, 2, 6, 24, 120, 720, 5040, 40320, 362880];
const MAX_WERT: usize = 7 * FAKULTAETEN[9];

#[inline(always)]
fn fakultaeten_summe(mut zahl: usize) -> usize {
    if zahl == 0 {
        return 0;
    }

    let mut summe = 0;

    while zahl > 0 {
        summe += FAKULTAETEN[zahl % 10];
        zahl /= 10;
    }

    summe
}

fn kettenlaenge(
    start: usize,
    speicher: &mut [u16],
    besucht_generation: &mut [u32],
    besucht_position: &mut [u16],
    generation: u32,
    pfad: &mut Vec<usize>,
) -> u16 {
    if speicher[start] != 0 {
        return speicher[start];
    }

    pfad.clear();
    let mut aktuell = start;

    loop {
        if speicher[aktuell] != 0 {
            let mut laenge = speicher[aktuell] + 1;

            for &wert in pfad.iter().rev() {
                speicher[wert] = laenge;
                laenge += 1;
            }

            return speicher[start];
        }

        if besucht_generation[aktuell] == generation {
            let kreis_start = besucht_position[aktuell] as usize;
            let kreis_laenge = (pfad.len() - kreis_start) as u16;

            for &wert in &pfad[kreis_start..] {
                speicher[wert] = kreis_laenge;
            }

            let mut laenge = kreis_laenge;

            for &wert in pfad[..kreis_start].iter().rev() {
                laenge += 1;
                speicher[wert] = laenge;
            }

            return speicher[start];
        }

        besucht_generation[aktuell] = generation;
        besucht_position[aktuell] = pfad.len() as u16;
        pfad.push(aktuell);

        aktuell = fakultaeten_summe(aktuell);
    }
}

fn main() {
    let mut speicher = vec![0u16; MAX_WERT + 1];
    let mut besucht_generation = vec![0u32; MAX_WERT + 1];
    let mut besucht_position = vec![0u16; MAX_WERT + 1];
    let mut pfad = Vec::with_capacity(64);

    let mut antwort = 0usize;
    let mut generation = 1u32;

    for zahl in 0..GRENZE {
        if kettenlaenge(
            zahl,
            &mut speicher,
            &mut besucht_generation,
            &mut besucht_position,
            generation,
            &mut pfad,
        ) == 60
        {
            antwort += 1;
        }

        generation += 1;
    }

    println!("{antwort}");
}