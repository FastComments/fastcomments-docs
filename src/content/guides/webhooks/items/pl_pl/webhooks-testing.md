W panelu administracyjnym Webhooks znajdują się przyciski `Send Test Payload` dla każdego typu zdarzenia (Create, Update, Delete). The Create and Update events send a dummy WebhookComment object, while testing Delete will send a dummy request body with just an ID.

Test wykona dwa wywołania, aby zweryfikować kod odpowiedzi dla scenariuszy "happy" (correct API Key) i "sad" (invalid API key).

Kiedy test wyśle nieprawidłowy API key, powinieneś zwrócić kod statusu 401, aby test przeszedł pomyślnie. Jeśli nie sprawdzisz poprawnie wartości tokena, otrzymasz błąd.

Ma to zapewnić, że poprawnie uwierzytelniasz żądanie.