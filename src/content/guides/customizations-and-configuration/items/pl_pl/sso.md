SSO, czyli single-sign-on, to zbiór konwencji używanych, aby umożliwić Tobie lub Twoim użytkownikom korzystanie z FastComments bez tworzenia kolejnego konta.

Zakładając, że nie dopuszczasz anonimowego komentowania, konto jest wymagane, aby komentować za pomocą FastComments. Upraszczamy ten proces rejestracji — użytkownik po prostu zostawia swój e-mail przy komentowaniu.
Rozumiemy jednak, że nawet to może być dodatkową barierą, której niektóre serwisy chcą uniknąć.

Możemy zredukować to utrudnienie, stosując tylko jeden przepływ logowania dla całej Twojej witryny.

### Jak to uzyskać?
Wszystkie typy kont obecnie mają dostęp do SSO. Jednak maksymalna liczba użytkowników SSO będzie się różnić w zależności od Twojego pakietu. Podobnie jak w przypadku innych funkcji, plany Pro i wyższe zapewniają bezpośrednie wsparcie deweloperskie.

Porównajmy opcje, a następnie przejdźmy do szczegółów każdej z nich.

### Migracje użytkowników i komentarzy

Podczas migracji z platformy ze SSO, takiej jak Disqus, będziesz już mieć użytkowników i ich komentarze.

Komentarze są importowane w ramach migracji, albo przez API, nasze narzędzie Import UI, albo przez wsparcie klienta. Import UI jest preferowany, jeśli obsługuje platformę, z której migrujesz, ponieważ zawiera obsługę błędów, ekstrakcję i przesyłanie avatarów oraz mediów oraz system monitorowania zadań wsadowych.

Sami użytkownicy są dodawani automatycznie przy pierwszym wyświetleniu wątków komentarzy. Alternatywnie mogą być wcześniej dodani za pomocą API, ale takie działanie nie daje wielu korzyści.

Jeśli komentarze są importowane, a użytkownicy SSO nie zostaną dodani ręcznie przez API, to komentarze zostaną automatycznie przypisane do kont użytkowników w momencie, gdy konto zostanie utworzone po ich pierwszym wyświetleniu dowolnego wątku komentarzy. Wówczas będą mogli zarządzać, edytować i usuwać komentarze, które pierwotnie napisali.

Automatyczna migracja odbywa się za pomocą e-maila lub nazwy użytkownika. Niektóre platformy nie udostępniają e-maili podczas eksportu, jak Disqus, więc w takim przypadku używamy nazwy użytkownika jako zastępstwa.
- Jeśli podasz pasującą nazwę użytkownika oraz e-mail w SSO payload, dodamy e-mail do poszczególnych obiektów komentarzy, aby powiadomienia i wzmianki działały.

Jeśli chcesz zaimportować komentarze i użytkowników jednocześnie, współpracuj ze wsparciem, aby przenieść komentarze do odpowiednich kont użytkowników po zaimportowaniu użytkowników przez API.

Podsumowując, najprostsza ścieżka migracji to:

1. Import komentarzy.
   1. Avatary i inne media są migrowane automatycznie, jeśli używasz Import UI w `Manage Data -> Imports`.
2. Skonfiguruj Secure lub Simple SSO.
3. Pozwól, aby migracja odbyła się automatycznie dla każdego użytkownika przy jego pierwszym logowaniu.
   1. Zazwyczaj dodaje to mniej niż sekundę do czasu ładowania strony, jeśli użytkownik ma mniej niż 50k komentarzy.

### Użytkownicy WordPress
Jeśli używasz naszej <a href="https://wordpress.org/plugins/fastcomments/" target="_blank">wtyczki WordPress</a>, nie musisz pisać żadnego kodu! Po prostu przejdź do strony administracyjnej wtyczki, kliknij SSO Settings, a następnie Enable.

Spowoduje to uruchomienie kreatora z jednym przyciskiem, który utworzy Twój klucz API, wyśle go do Twojej instalacji WordPress i włączy SSO. Uprościliśmy to dla Ciebie do jednego kliknięcia.

Zwróć uwagę, że jeśli instalujesz wtyczkę po raz pierwszy, będziesz musiał przejść proces konfiguracji, zanim zobaczysz stronę administracyjną z przyciskiem SSO Settings.

#### WordPress SSO - Moderatorzy

Zauważ, że obecnie aby obok komentarzy Twoich moderatorów pojawiła się odznaka "Moderator" podczas komentowania przy użyciu wtyczki FastComments dla WordPress,
musi on być również dodany jako Moderator w panelu FastComments oraz mieć zweryfikowany e-mail.

### Integracje niestandardowe

Dla integracji niestandardowych istnieją dwie opcje.

### Opcja pierwsza - Secure SSO

W przypadku Secure SSO FastComments wie, że użytkownik komentujący, głosujący i czytający komentarze jest prawdziwym użytkownikiem na Twojej stronie.

Jeśli utworzysz prawidłowy payload, użytkownik zawsze będzie miał płynne doświadczenie komentowania.

W Secure SSO payload SSO jest tworzony po stronie **server-side** przy użyciu uwierzytelniania HMAC, a następnie przekazywany do widżetu po stronie **client**.

W Secure SSO konto użytkownika jest **całkowicie oddzielone** od reszty bazy użytkowników FastComments. Oznacza to, że jeśli mamy dwóch partnerów
Company A i Company B, każdy z nich może mieć użytkownika SSO o nazwie użytkownika "Bob".

#### Wymagania
- Podstawowa wiedza z zakresu tworzenia backendu.
- Podstawowa wiedza dotycząca obchodzenia się z tajnymi kluczami API.
- Podstawowa wiedza dotycząca tworzenia API lub renderowania po stronie serwera.

#### Zalety
- Bezpieczne.
- Płynne doświadczenie komentowania.

#### Wady
- Wymaga prac backendowych.

#### Aktualizowanie danych użytkownika

W Secure SSO za każdym razem, gdy przekażesz sso user payload, zaktualizujemy użytkownika do najnowszych informacji. Na przykład, jeśli
użytkownik ma nazwę użytkownika `X`, a Ty przekażesz `Y` w SSO payload, jego nazwa użytkownika zmieni się na `Y`.

Jeśli chcesz usunąć wartości przy użyciu tego podejścia, ustaw je na `null` (nie `undefined`).

#### Secure SSO API

Udostępniamy również API do interakcji z użytkownikami SSO. Zobacz [the docs](/guide-api.html#sso-user-structure).

Zwróć uwagę, że przy użyciu Secure SSO, użytkownicy są tworzeni automatycznie w tle podczas ładowania strony. Nie musisz masowo importować swoich użytkowników.

### Opcja druga - Simple SSO

Alternatywą dla Secure SSO jest po prostu przekazanie informacji o użytkowniku do widżetu komentarzy.

Podanie e-maila przy Simple SSO nie jest wymagane, jednak bez niego ich komentarze będą wyświetlane jako "Unverified".

<sup>Uwaga!</sup> Od początku 2022 nazwy użytkowników w Simple SSO nie muszą być unikalne w całym serwisie FastComments.com.

Idealnie Simple SSO powinno być wybierane tylko podczas tworzenia na platformie, która nie daje dostępu do backendu.

#### Wymagania
- Podstawowa wiedza z zakresu programowania po stronie klienta.
- Znajomość przynajmniej e-maila użytkownika.

#### Zalety
- Proste.
- Wszystkie działania są nadal weryfikowane.
- Użytkownik nigdy nie wprowadza swojej nazwy użytkownika ani e-maila.

#### Wady
- Mniej bezpieczne niż Secure SSO, ponieważ payload po stronie klienta może być spreparowany, aby podszyć się pod dowolnego użytkownika.

#### Simple SSO API

Użytkownicy tworzeni automatycznie przez przepływ Simple SSO są przechowywani jako obiekty `SSOUser`. Można uzyskać do nich dostęp i nimi zarządzać za pomocą API `SSOUser`. Zobacz [the docs](/guide-api.html#sso-user-structure).