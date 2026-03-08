Canvas-roller kortlægges automatisk til FastComments-roller under LTI-starten. Ingen manuel konfiguration er nødvendig.

#### Rollekortlægning

| Canvas Role | FastComments Role | Permissions |
|---|---|---|
| **Administrator** | Admin | Fuld konto-adgang, administrer alle kommentarer og indstillinger |
| **Instructor** | Moderator | Rediger og slet kommentarer, fastgør tråde, administrer diskussioner |
| **Learner** | Commenter | Skriv kommentarer, svar, stem, og brug omtaler |

#### Sådan fungerer det

Når en bruger starter FastComments fra Canvas, inkluderer LTI 1.3-protokollen deres Canvas-rolle. FastComments læser denne rolle og tildeler de passende tilladelser automatisk.

Hvis en bruger har flere roller (f.eks. en Instruktør, som også er Admin), bruges den rolle med højeste privilegium.