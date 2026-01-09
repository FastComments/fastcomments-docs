Często stosuje się osobny sub-tenant dla każdego środowiska testowego lub deweloperskiego w FastComments. Każdy tenant ma własną konfigurację, dane i klucze API. Konfiguracja, dane i użytkownicy nie mogą być współdzieleni między tenantami.
Wszystko jest izolowane. Jednak super administratorzy tenanta nadrzędnego mogą podszywać się pod użytkowników w tenantach potomnych.

Istnieją dwa podejścia:

- Główny tenant jest przeznaczony dla produkcji, a sub-tenanty dla środowisk testowych.
- Główny tenant służy wyłącznie do rozliczeń, a każdy sub-tenant jest dla prod, test i tak dalej.

Pierwsze podejście jest zazwyczaj łatwiejsze do zrozumienia dla użytkowników, ale może to zależeć od waszej organizacji.

Tenanty można tworzyć [tutaj](https://eu.fastcomments.com/auth/my-account/tenants) jeśli macie pakiet. To jest również miejsce, gdzie super administratorzy będą
podszywać się pod użytkowników. Tenanty można też tworzyć za pomocą API w celu bardziej niestandardowych/zautomatyzowanych konfiguracji.

Niezależnie od wybranego podejścia, będziesz musiał dodać moderatorów i użytkowników, którzy chcą widzieć dane produkcyjne w tenancie "prod". Na przykład, jeśli chcesz
wybrać opcję B i użyć tenanta nadrzędnego do rozliczeń, oraz mieć sub-tenant dla "prod", powinieneś dodać tenanta, przełączyć się na nowy tenanta i dodać swoich
administratorów i moderatorów dla sub-tenanta. 

Na koniec, aby wyjaśnić, strona "Moderate Comments" będzie pusta w przypadku opcji B dla tenanta nadrzędnego.