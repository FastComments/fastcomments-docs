Narzędzie Warn wysyła prywatne ostrzeżenie DM do użytkownika dotyczące konkretnego komentarza, a jednocześnie zapisuje ostrzeżenie w współdzielonej [pamięci agenta](#agent-memory-system). Oba zapisy są atomowe — użytkownik nigdy nie widzi ostrzeżenia, które nie zostało również zanotowane.

### Dlaczego istnieje

Polityka eskalacji platformy to **najpierw ostrzeżenie, zablokowanie tylko jeśli użytkownik ponownie naruszy zasady**. Narzędzie Warn sprawia, że ta polityka jest wykonalna: daje użytkownikowi szansę na poprawę, a zapis ostrzeżenia to to, co znajdzie przyszły agent, gdy przeszuka pamięć przed rozważeniem zablokowania.

Narzędzie także eliminuje duplikaty: jeśli agent już wydał ostrzeżenie temu samemu użytkownikowi dotyczące tego samego komentarza, drugie ostrzeżenie nie ma efektu. Dzięki temu LLM, który zapętla się lub ponownie uruchamia w związku z tym samym komentarzem, nie może spamować użytkownika wieloma ostrzeżeniami.

### Co powinno znaleźć się w ostrzeżeniu

Krótka wiadomość (ograniczona do 1000 znaków) wyświetlana użytkownikowi jako DM. Skuteczne ostrzeżenia są:

- **Konkretne** - „Osobiste ataki na wymienionych użytkowników nie są dozwolone w tej społeczności” jest lepsze niż „Twój komentarz został zgłoszony.”
- **Zwięzłe** - maksymalnie kilka zdań.
- **Praktyczne** - powiedz użytkownikowi, co ma zmienić. „Proszę edytuj swój komentarz, usuń nazwę użytkownika, inaczej zostanie usunięty.”

Nie piszesz tej wiadomości samodzielnie; robi to agent na podstawie [początkowej podpowiedzi](#personality-prompt) i [wytycznych społeczności](#community-guidelines). Twoim zadaniem jest napisanie podpowiedzi, która wygeneruje dobre ostrzeżenia.

### Kiedy zezwalać na jego użycie

Dla każdego agenta o charakterze moderacyjnym. Szablon Moderator włącza je domyślnie.

### Zatwierdzenia

Rzadziej objęte blokadą niż [Zbanuj użytkownika](#tool-ban-user). Warto stosować blokadę w ciągu pierwszych tygodni działania agenta, aby móc wykryć złe ostrzeżenia zanim zostaną wysłane, ale większość operatorów usuwa blokadę, gdy agent zaczyna generować wiarygodne wyniki.

### Zobacz także

- [Zbanuj użytkownika](#tool-ban-user) - kolejny krok w eskalacji.
- [System pamięci agenta](#agent-memory-system) - miejsce przechowywania zapisów ostrzeżeń.