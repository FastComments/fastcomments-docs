---
Moduł FastComments dla Drupala zastępuje wbudowany system komentarzy Drupala szybkim, działającym w czasie rzeczywistym systemem komentarzy. Moduł jest [opublikowany na drupal.org](https://www.drupal.org/project/fcom) i działa z Drupalem 10 i 11.

Istnieją dwa sposoby instalacji.

## Instalacja za pomocą Composera

```
composer require drupal/fcom
drush en fastcomments
```

## Instalacja ręczna

Pobierz moduł z [drupal.org/project/fcom](https://www.drupal.org/project/fcom) i umieść go w katalogu `modules/custom/fastcomments/` swojej witryny. Następnie włącz go za pomocą `drush en fastcomments`, lub z poziomu interfejsu administracyjnego w `Extend` (`/admin/modules`).

<sup>Note!</sup> Moduł zależy tylko od rdzenia Drupala (`user` i `field`). Nie są wymagane żadne inne moduły Drupala ani biblioteki.

Po włączeniu modułu przejdź do sekcji `Configuration`, aby skonfigurować swój Tenant ID i API Secret.

---