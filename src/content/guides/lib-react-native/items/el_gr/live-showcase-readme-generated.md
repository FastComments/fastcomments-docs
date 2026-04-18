Για να δείτε κάθε widget και ροή να εκτελείται τοπικά ενάντια στον δημόσιο `demo` tenant, κλωνοποιήστε το αποθετήριο και εκτελέστε:

```bash
yarn bootstrap
cd example
yarn ios       # or: yarn android, yarn web
```

Το σημείο εισόδου του showcase είναι `example/src/ShowcaseApp.tsx` — μια ενιαία εφαρμογή που εκθέτει όλα τα widgets, θέματα και ροές.

Ο στόχος `yarn web` χρησιμοποιεί `react-native-web` + `react-native-web-webview` (που αποδίδει το WebView ως iframe). Χρήσιμο για γρήγορους οπτικούς ελέγχους (smoke tests) σε έναν περιηγητή· τα native-only WebView APIs όπως `injectJavaScript` και `onShouldStartLoadWithRequest` δεν θα συμπεριφέρονται πλήρως στο web.