#### Wzmiankowanie użytkowników z innych grup

Jeśli dwaj użytkownicy należą do dwóch różnych zbiorów grup i nie ma między nimi przecięcia, nie będą mogli się nawzajem `@mention`.

Jeśli użytkownik ręcznie wpisze `@mention` i wyśle swój komentarz, pozostanie on w postaci zwykłego tekstu. Inny użytkownik nie zostanie otagowany.

#### Zarządzanie grupami

`Groups` są definiowane przy użyciu zasobów API `Pages` i `SSOUsers`.

API `Pages` można wywołać, aby zdefiniować zestaw grup mających dostęp do strony. Domyślnie dostęp mają wszystkie grupy oraz użytkownicy, którzy nie należą do żadnej grupy.

Podobnie API `SSOUsers` można wywołać, aby zdefiniować grupy powiązane z każdym użytkownikiem.

Dla obu zasobów nie ma ograniczeń co do tego, kiedy można ustawić lub zaktualizować grupy.

Jeśli celem jest jedynie ograniczenie możliwości `@mention`owania się użytkowników nawzajem, to `Pages` nie muszą być brane pod uwagę.

##### Uwaga!

Definiowanie i aktualizowanie grup użytkowników SSO nie wymaga używania API i może być zaktualizowane automatycznie przez zdefiniowanie identyfikatorów grup w ładunku SSO przekazywanym do widżetu komentarzy. Jednak przy dużych listach grup nie jest to zalecane, ponieważ użytkownik musiałby przesyłać ten ładunek przy każdym ładowaniu strony.