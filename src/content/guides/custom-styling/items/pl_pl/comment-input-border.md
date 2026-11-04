Obramowanie wokół pola komentarza składa się z własnej ramki elementu `textarea` oraz kilku cienkich linii rysowanych przez widżet wokół niego. Aby zmienić jego kolor lub zaokrąglenie narożników, ustaw te zmienne CSS zamiast stylować `textarea` bezpośrednio. Zmienne restylizują całe obramowanie naraz, dzięki czemu narożniki i kolory zawsze się zgadzają.

| Zmienna | Co zmienia | Domyślnie |
|---|---|---|
| `--fc-input-border-color` | Kolor obramowania | `#bfbfbf` |
| `--fc-input-border-color-focus` | Kolor obramowania podczas pisania | `#555` |
| `--fc-input-border-radius` | Zaokrąglenie zaokrąglonych narożników | `11px` |
| `--fc-input-border-start-start-radius` | Zaokrąglenie kwadratowego lewego górnego narożnika (prawego górnego w językach pisanych od prawej do lewej) | `0` |

Dodaj CSS do pola **Custom CSS** na [stronie dostosowywania widżetu](https://fastcomments.com/auth/my-account/customize-widget) lub przekaż go za pomocą opcji `customCSS`. Musisz ustawić tylko te zmienne, które chcesz zmienić.

Możesz również użyć pomocnika **Comment box border** znajdującego się tuż pod polem Custom CSS, który wygeneruje ten CSS za Ciebie.

## Zmiana koloru obramowania

[inline-code-attrs-start title = 'Kolor obramowania'; type = 'css'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
:root {
    --fc-input-border-color: #d1d5db;
}
[inline-code-end]

## Zmiana koloru obramowania podczas pisania

[inline-code-attrs-start title = 'Kolor obramowania podczas pisania'; type = 'css'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
:root {
    --fc-input-border-color-focus: #2563eb;
}
[inline-code-end]

## Zaokrąglenie wszystkich czterech narożników

Domyślnie lewy górny narożnik jest kwadratowy. Ustaw obie zmienne promienia, aby zaokrąglić wszystkie cztery narożniki tak samo:

[inline-code-attrs-start title = 'Zaokrąglenie wszystkich czterech narożników'; type = 'css'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
:root {
    --fc-input-border-radius: 12px;
    --fc-input-border-start-start-radius: 12px;
}
[inline-code-end]

## Kwadratowe narożniki

[inline-code-attrs-start title = 'Kwadratowe narożniki'; type = 'css'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
:root {
    --fc-input-border-radius: 0;
}
[inline-code-end]

## Dopasowanie do marki

[inline-code-attrs-start title = 'Kolory marki i narożniki'; type = 'css'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
:root {
    --fc-input-border-color: #c7d2fe;
    --fc-input-border-color-focus: #4f46e5;
    --fc-input-border-radius: 8px;
    --fc-input-border-start-start-radius: 8px;
}
[inline-code-end]

## Różne kolory w trybie ciemnym

Gdy widżet jest w trybie ciemnym, ma klasę `dark`, więc możesz ustawić różne wartości dla trybu ciemnego:

[inline-code-attrs-start title = 'Kolory obramowania w trybie ciemnym'; type = 'css'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
:root {
    --fc-input-border-color: #d1d5db;
}
.dark {
    --fc-input-border-color: #444;
    --fc-input-border-color-focus: #aaa;
}
[inline-code-end]

## Dlaczego nie stylować pola tekstowego bezpośrednio?

Widżet rysuje część obramowania pola komentarza samodzielnie, wokół pola tekstowego. Jeśli ustawisz `border-color` lub `border-radius` tylko na `textarea`, te linie zachowają styl domyślny i obramowanie będzie wyglądać niespójnie, na przykład kwadratowa linia przechodząca przez zaokrąglony narożnik. Powyższe zmienne zmieniają oba elementy naraz.