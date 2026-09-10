FastComments podržava webhook‑ove samo za resurs Komentar.

Podržavamo webhook‑ove za kreiranje komentara, brisanje i ažuriranje.

Svaki od ovih smatra se posebnim događajem u našem sistemu i kao takav ima različitu semantiku  
i strukture za webhook događaje.

Bilo koji broj krajnjih tačaka može da se pretplati na isti događaj, putem kontrolne table ili kroz API  
(vidi Upravljanje webhook‑ovima putem API‑ja). Svaki webhook se isporučuje nezavisno.