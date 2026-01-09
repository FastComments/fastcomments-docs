FastComments αντιστοιχίζει τους SAML ρόλους χρηστών σε εσωτερικά δικαιώματα, επιτρέποντας έλεγχο πρόσβασης με βάση τους ρόλους για τον οργανισμό σας.

### FastComments Role System

Το FastComments χρησιμοποιεί ένα σύστημα αδειών με βάση τους ρόλους όπου οι χρήστες μπορούν να έχουν έναν ή περισσότερους ρόλους που καθορίζουν τα επίπεδα πρόσβασης και τις δυνατότητές τους.

### Available FastComments Roles

#### Administrative Roles

**fc-account-owner**
- **Permissions**: Πλήρης διοικητική πρόσβαση
- **Capabilities**: Όλες οι λειτουργίες, διαχείριση χρεώσεων, διαχείριση χρηστών
- **Use Case**: Κύριοι διαχειριστές λογαριασμού και ιδιοκτήτες

**fc-admin-admin**  
- **Permissions**: Διοικητική πρόσβαση στις περισσότερες λειτουργίες
- **Capabilities**: Διαχείριση χρηστών, διαμόρφωση, μέτρηση και διαχείριση περιεχομένου. **Μπορεί να διαχειρίζεται άλλους διαχειριστές.**
- **Use Case**: Δευτερεύοντες διαχειριστές και προσωπικό IT

**fc-billing-admin**
- **Permissions**: Διαχείριση χρεώσεων και συνδρομών
- **Capabilities**: Τρόποι πληρωμής, τιμολόγια, αλλαγές συνδρομής
- **Use Case**: Μέλη ομάδας οικονομικών και υπεύθυνοι χρεώσεων

#### Specialized Roles

**fc-analytics-admin**
- **Permissions**: Πρόσβαση σε αναλύσεις και αναφορές
- **Capabilities**: Προβολή στατιστικών ιστότοπου, δεδομένα εμπλοκής χρηστών
- **Use Case**: Ομάδες μάρκετινγκ και αναλυτές δεδομένων

**fc-api-admin**
- **Permissions**: Πρόσβαση και διαχείριση API
- **Capabilities**: Διαπιστευτήρια API, διαμόρφωση webhooks
- **Use Case**: Προγραμματιστές και τεχνικοί ενσωματωτές

**fc-moderator**
- **Permissions**: Δυνατότητες διαχείρισης σχολίων
- **Capabilities**: Έγκριση/απόρριψη σχολίων, διαχείριση ανεπιθύμητου περιεχομένου
- **Use Case**: Συντονιστές κοινοτήτων και διαχειριστές περιεχομένου

### Role Mapping Configuration

#### SAML Attribute Sources

Το FastComments δέχεται πληροφορίες ρόλων από διάφορα ονόματα χαρακτηριστικών SAML για να εξασφαλίσει συμβατότητα με διαφορετικούς παρόχους ταυτότητας:

**Standard Attribute Names**:
- `roles`
- `groups` 
- `memberOf`
- `role`
- `group`

**Microsoft/ADFS Attributes**:
- `http://schemas.microsoft.com/ws/2008/06/identity/claims/role`
- `http://schemas.xmlsoap.org/ws/2005/05/identity/claims/role`

#### Role Format Support

**Array Format** *(Preferred)*:
```xml
<saml:Attribute Name="roles">
    <saml:AttributeValue>fc-admin-admin</saml:AttributeValue>
    <saml:AttributeValue>fc-moderator</saml:AttributeValue>
</saml:Attribute>
```

**Comma-Separated Format**:
```xml
<saml:Attribute Name="roles">
    <saml:AttributeValue>fc-admin-admin,fc-moderator</saml:AttributeValue>
</saml:Attribute>
```

**Single Role Format**:
```xml
<saml:Attribute Name="roles">
    <saml:AttributeValue>fc-admin-admin</saml:AttributeValue>
</saml:Attribute>
```

### Identity Provider Role Configuration

#### Microsoft Azure AD

1. **App Roles Configuration**:
   - Ορίστε τους ρόλους FastComments στην εφαρμογή Azure AD σας
   - Αναθέστε χρήστες στους κατάλληλους ρόλους εφαρμογής
   - Διαμορφώστε τα claims ώστε να περιλαμβάνουν τους ανατεθέντες ρόλους

2. **Attribute Mapping**:
   ```
   Attribute Name: roles
   Source Attribute: user.assignedroles
   ```

#### Okta

1. **Group Assignment**:
   - Δημιουργήστε ομάδες που ταιριάζουν με τα ονόματα ρόλων του FastComments
   - Αναθέστε χρήστες στις κατάλληλες ομάδες
   - Διαμορφώστε δηλώσεις χαρακτηριστικών

2. **Attribute Statement**:
   ```
   Name: roles
   Value: user.groups
   Filter: Starts with "fc-"
   ```

#### Google Workspace

1. **Group Mapping**:
   - Δημιουργήστε οργανωτικές μονάδες ή ομάδες
   - Ονομάστε τις ομάδες με τα πρόθεματα ρόλων του FastComments
   - Διαμορφώστε την αντιστοίχιση χαρακτηριστικών

2. **Custom Attributes**:
   ```
   Attribute Name: roles
   Value: Groups or custom schema attribute
   ```

### Default User Behavior

#### Users Without Roles

Όταν ένας χρήστης SAML δεν έχει ρόλους ή έχει μη αναγνωρισμένους ρόλους:
- Ο χρήστης δημιουργείται ως κανονικός σχολιαστής
- Δεν χορηγείται διοικητική πρόσβαση
- Μπορεί να δημοσιεύει και να διαχειρίζεται τα δικά του σχόλια
- Δεν μπορεί να έχει πρόσβαση σε δυνατότητες του διαχειριστικού πίνακα

#### Role Inheritance

- Οι χρήστες μπορούν να έχουν πολλαπλούς ρόλους ταυτόχρονα
- Τα δικαιώματα είναι σωρευτικά (ισχύει το υψηλότερο επίπεδο δικαιωμάτων)
- Οι αλλαγές ρόλων στον IdP αντικατοπτρίζονται στην επόμενη σύνδεση

### Managing SAML Users

#### User Creation

Όταν ένας χρήστης συνδέεται μέσω SAML για πρώτη φορά:
1. **User Account**: Δημιουργείται αυτόματα με το email ως αναγνωριστικό
2. **Role Assignment**: Οι ρόλοι εφαρμόζονται βάσει των χαρακτηριστικών SAML
3. **Profile Information**: Το όνομα/επώνυμο συμπληρώνονται αν παρέχονται
4. **Permission Activation**: Οι ρόλοι γίνονται ενεργοί άμεσα

#### Role Updates

Οι υπάρχοντες χρήστες SAML λαμβάνουν ενημερώσεις ρόλων:
1. **Login Trigger**: Οι ενημερώσεις ρόλων πραγματοποιούνται κατά κάθε σύνδεση SAML
2. **Immediate Effect**: Τα νέα δικαιώματα εφαρμόζονται αμέσως
3. **Role Removal**: Οι αφαιρεθέντες ρόλοι ανακαλούνται αυτόματα
4. **Audit Trail**: Οι αλλαγές ρόλων καταγράφονται στα αρχεία ελέγχου

### Custom Role Mapping

#### Enterprise Customization

Για εταιρικούς πελάτες με συγκεκριμένες απαιτήσεις:
- Προσαρμοσμένα ονόματα ρόλων μπορούν να αντιστοιχιστούν σε δικαιώματα FastComments
- Μπορούν να υλοποιηθούν πολύπλοκες ιεραρχίες ρόλων
- Μπορούν να διαμορφωθούν έλεγχοι πρόσβασης ανά τμήμα

Επικοινωνήστε με την υποστήριξη FastComments για διαμορφώσεις προσαρμοσμένης αντιστοίχισης ρόλων.

#### Role Validation

Το FastComments επικυρώνει τους εισερχόμενους ρόλους:
- Οι μη αναγνωρισμένοι ρόλοι αγνοούνται (δεν απορρίπτονται)
- Τα εσφαλμένα μορφοποιημένα χαρακτηριστικά ρόλων καταγράφονται για αντιμετώπιση προβλημάτων
- Οι χρήστες διατηρούν τους υπάρχοντες ρόλους εάν η δήλωση SAML δεν περιέχει πληροφορίες ρόλων

### Best Practices

#### Role Management

1. **Principle of Least Privilege**: Αναθέστε τα ελάχιστα απαραίτητα δικαιώματα
2. **Regular Auditing**: Ελέγχετε τακτικά τους ρόλους και την πρόσβαση των χρηστών  
3. **Clear Naming**: Χρησιμοποιήστε περιγραφικά ονόματα ομάδων στον IdP σας
4. **Documentation**: Διατηρήστε τεκμηρίωση των αναθέσεων ρόλων

#### Security Considerations

1. **Role Attributes**: Διασφαλίστε ότι τα χαρακτηριστικά ρόλων προστατεύονται σωστά στις απαντήσεις SAML
2. **Attribute Validation**: Επαληθεύστε ότι μόνο εξουσιοδοτημένα συστήματα μπορούν να αναθέτουν ρόλους
3. **Access Reviews**: Ελέγχετε τακτικά τις αναθέσεις διοικητικών ρόλων
4. **Monitoring**: Παρακολουθείτε τις αλλαγές ρόλων και τις διοικητικές ενέργειες

### Troubleshooting Role Issues

#### Common Problems

**Roles Not Applied**:
- Ελέγξτε ότι τα ονόματα χαρακτηριστικών SAML ταιριάζουν με τις υποστηριζόμενες μορφές
- Επαληθεύστε ότι ο IdP στέλνει πληροφορίες ρόλων
- Επιβεβαιώστε ότι οι τιμές ρόλων ταιριάζουν ακριβώς με τα ονόματα ρόλων του FastComments

**Access Denied**:
- Επαληθεύστε ότι ο χρήστης έχει τον κατάλληλο ρόλο ανατεθειμένο στον IdP
- Ελέγξτε την ορθογραφία και την ευαισθησία πεζών/κεφαλαίων των ρόλων
- Επιβεβαιώστε ότι ο ρόλος είναι σωστά μορφοποιημένος στην απάντηση SAML

**Missing Permissions**:
- Επανεξετάστε τους ορισμούς ρόλων και τα απαιτούμενα δικαιώματα
- Ελέγξτε για αντικρουόμενες αναθέσεις ρόλων
- Επαληθεύστε ότι ο χρήστης έχει συνδεθεί μετά από αλλαγές ρόλων

---