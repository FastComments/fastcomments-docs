Στο Webhooks admin υπάρχουν κουμπιά `Send Test Payload` για κάθε τύπο συμβάντος (Create, Update, Delete). Τα Create και Update συμβάντα στέλνουν ένα dummy αντικείμενο `WebhookComment`, ενώ η δοκιμή του Delete θα στείλει ένα dummy σώμα αιτήματος με μόνο ένα ID.

## Verifying Payloads

Κατά τη δοκιμή της ενσωμάτωσης webhook, επαληθεύστε ότι τα εισερχόμενα αιτήματα περιλαμβάνουν τις ακόλουθες κεφαλίδες:

1. **`token`** - Το μυστικό API σας
2. **`X-FastComments-Timestamp`** - Unix χρονική σφραγίδα (δευτερόλεπτα)
3. **`X-FastComments-Signature`** - HMAC-SHA256 υπογραφή

Χρησιμοποιήστε την επαλήθευση υπογραφής HMAC για να διασφαλίσετε ότι τα payloads είναι γνήσια.

## Testing Tools

Μπορείτε να χρησιμοποιήσετε εργαλεία όπως [webhook.site](https://webhook.site) ή [ngrok](https://ngrok.com) για να εξετάσετε τα εισερχόμενα payloads webhook κατά την ανάπτυξη.

## Event Types

- **Create Event**: Προκαλείται όταν δημιουργείται ένα νέο σχόλιο. Προεπιλεγμένη μέθοδος: PUT
- **Update Event**: Προκαλείται όταν επεξεργάζεται ένα σχόλιο. Προεπιλεγμένη μέθοδος: PUT
- **Delete Event**: Προκαλείται όταν διαγράφεται ένα σχόλιο. Προεπιλεγμένη μέθοδος: DELETE

Κάθε συμβάν περιλαμβάνει τα πλήρη δεδομένα του σχολίου στο σώμα του αιτήματος (βλέπε [Data Structures](/guide-webhooks.html#webhooks-structures) για τη μορφή του payload).

---