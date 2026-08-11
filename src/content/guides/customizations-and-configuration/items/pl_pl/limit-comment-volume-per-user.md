---
Domyślnie każdy użytkownik może dodać maksymalnie `5 komentarzy` w ciągu jednej minuty.

Jest to śledzone za pomocą identyfikatora użytkownika, anonimowego identyfikatora użytkownika oraz adresu IP (zaszyfrowanego).

Można to dostosować bez kodu, na stronie dostosowywania widżetu:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.max-comments-per-minute'; alt='Pole maksymalnej liczby komentarzy na minutę na stronie dostosowywania widżetu, domyślnie ustawione na 5'; title='Ograniczanie liczby komentarzy na użytkownika' app-screenshot-end]

Zauważ, że jeśli używasz API tworzenia komentarzy, możesz chcieć przekazać oryginalny adres `ip` użytkownika w żądaniu do naszego backendu, aby limitowanie było stosowane
per użytkownika, a nie globalnie dla Twojego konta.

---