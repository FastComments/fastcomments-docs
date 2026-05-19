Blackboard Learn SaaS and Ultra support LTI 1.3 Dynamic Registration.

#### Άνοιγμα της οθόνης Tool Provider

1. Κάντε είσοδο στο Blackboard ως διαχειριστής συστήματος.
2. Μεταβείτε στο **Administrator Panel** > **Integrations** > **LTI Tool Providers**.
3. Κάντε κλικ στο **Register LTI 1.3 / LTI Advantage Tool**.

If you only see "Register LTI 1.1 Provider", your Blackboard version doesn't support LTI 1.3 yet - upgrade or contact Blackboard support.

#### Επικόλληση του URL

Paste the FastComments registration URL (<a href="https://fastcomments.com/auth/my-account/lti-config" target="_blank">λάβετε το εδώ</a>) into the **Client ID** / **Registration URL** field (Blackboard's labeling varies by version). Submit.

Blackboard performs the registration handshake with FastComments and shows you a confirmation screen.

#### Έγκριση και Ενεργοποίηση

Blackboard marks newly-registered tools as **Approved but excluded** by default:

1. Βρείτε την καταχώρηση FastComments στη λίστα παρόχων εργαλείων.
2. Ανοίξτε το μενού και επιλέξτε **Edit**.
3. Ορίστε το **Tool Status** σε **Approved**.
4. Στην ενότητα **Institution Policies**, ελέγξτε ποια δεδομένα χρήστη αποστέλλονται (όνομα, email, ρόλος). Αποθηκεύστε.

The tool is now available to instructors when they add content to courses.