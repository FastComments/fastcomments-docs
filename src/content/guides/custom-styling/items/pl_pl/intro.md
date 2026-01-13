Ten przewodnik zawiera kompletny domyślny CSS używany przez widget komentowania FastComments (v2) oraz kilka instrukcji dotyczących nadpisywania stylów.

Zrozumienie domyślnego CSS pozwala Ci na:

- **Dostosować wygląd** poprzez nadpisywanie konkretnych stylów
- **Rozwiązywać problemy ze stylowaniem** poprzez sprawdzenie, jakie klasy i selektory są dostępne
- **Tworzyć własne motywy**, które będą działać ze strukturą widgetu
- **Korzystać z asystentów AI** do generowania niestandardowych modyfikacji CSS.

## Jak nadpisać style

Sposób nadpisywania stylów zależy od widgetu. Dla widgetu komentarzy musisz użyć parametru konfiguracji `customCSS`, aby przekazać CSS do iframe, lub określić CSS na stronie Dostosowywania w panelu administracyjnym, która będzie serwować CSS z naszego CDN.

---