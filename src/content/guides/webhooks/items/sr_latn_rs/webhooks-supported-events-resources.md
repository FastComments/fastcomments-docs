---
FastComments podržava webhook‑ove samo za resurs **Comment**.

Podržavamo webhook‑ove za kreiranje komentara, uklanjanje i ažuriranje.

Svaki od ovih se smatra posebnim događajem u našem sistemu i kao takav ima različitu semantiku  
i strukture za webhook događaje.

Bilo koji broj krajnjih tačaka može da se pretplati na isti događaj: jedan webhook po domenu može da se konfiguriše u  
kontrolnoj tabli, a dodatne pretplate mogu da se kreiraju putem API‑ja (vidi **Managing Webhooks via the API**).
---