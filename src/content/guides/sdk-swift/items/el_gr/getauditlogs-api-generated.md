## Παράμετροι

| Όνομα | Τύπος | Τοποθεσία | Απαιτείται | Περιγραφή |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes | Ο ενοικιαστής για τον οποίο διαβάζονται τα αρχεία ελέγχου. |
| limit | integer | query | No | Πόσα αρχεία ελέγχου να επιστραφούν. Μέγιστο 10000, προεπιλογή 1000. |
| skip | integer | query | No | Πόσα αρχεία ελέγχου να παραληφθούν, για σελιδοποίηση. |
| order | string | query | No | Κατεύθυνση ταξινόμησης με βάση το χρόνο του γεγονότος. Προεπιλογή DESC (πρώτα τα πιο πρόσφατα). |
| after | integer | query | No |  |
| before | integer | query | No |  |
| username | string | query | No | Να επιστραφούν μόνο τα γεγονότα που εκτελέστηκαν από αυτό το όνομα χρήστη. |
| ip | string | query | No | Να επιστραφούν μόνο τα γεγονότα από αυτή τη διεύθυνση IP. |
| crudType | string | query | No | Να επιστραφούν μόνο τα γεγονότα αυτού του τύπου: c, r, u, d ή login. |
| resourceName | string | query | No | Να επιστραφούν μόνο τα γεγονότα για αυτόν τον πόρο, π.χ. User ή Moderator. |
| targetId | string | query | No | Να επιστραφούν μόνο τα γεγονότα των οποίων το αντικείμενο-στόχος έχει αυτό το αναγνωριστικό. |
| target | string | query | No | Αναζήτηση υποσυμβολοσειράς χωρίς διάκριση πεζών-κεφαλαίων στο ετικέτα του αντικειμένου-στόχου, π.χ. όνομα χρήστη ή email. |
| includeManagedTenants | boolean | query | No | Να επιστραφούν επίσης γεγονότα από ενοικιαστές που διαχειρίζεται αυτός ο ενοικιαστής. |

## Απάντηση

Επιστρέφει: [`GetAuditLogsResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/GetAuditLogsResponse.swift)

## Παράδειγμα

[inline-code-attrs-start title = 'Παράδειγμα getAuditLogs'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Τα παρακάτω δείγματα κώδικα είναι ακόμη σε beta. Για οποιοδήποτε πρόβλημα, παρακαλούμε αναφέρετέ το μέσω http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | Ο ενοικιαστής για τον οποίο διαβάζονται τα αρχεία ελέγχου.
let limit = 987 // Int | Πόσα αρχεία ελέγχου να επιστραφούν. Μέγιστο 10000, προεπιλογή 1000. (optional)
let skip = 987 // Int | Πόσα αρχεία ελέγχου να παραληφθούν, για σελιδοποίηση. (optional)
let order = SORT_DIR() // SORTDIR | Κατεύθυνση ταξινόμησης με βάση το χρόνο του γεγονότος. Προεπιλογή DESC (πρώτα τα πιο πρόσφατα). (optional)
let after = 987 // Int64 |  (optional)
let before = 987 // Int64 |  (optional)
let username = "username_example" // String | Να επιστραφούν μόνο τα γεγονότα που εκτελέστηκαν από αυτό το όνομα χρήστη. (optional)
let ip = "ip_example" // String | Να επιστραφούν μόνο τα γεγονότα από αυτή τη διεύθυνση IP. (optional)
let crudType = "crudType_example" // String | Να επιστραφούν μόνο τα γεγονότα αυτού του τύπου: c, r, u, d ή login. (optional)
let resourceName = "resourceName_example" // String | Να επιστραφούν μόνο τα γεγονότα για αυτόν τον πόρο, π.χ. User ή Moderator. (optional)
let targetId = "targetId_example" // String | Να επιστραφούν μόνο τα γεγονότα των οποίων το αντικείμενο-στόχος έχει αυτό το αναγνωριστικό. (optional)
let target = "target_example" // String | Αναζήτηση υποσυμβολοσειράς χωρίς διάκριση πεζών-κεφαλαίων στο ετικέτα του αντικειμένου-στόχου, π.χ. όνομα χρήστη ή email. (optional)
let includeManagedTenants = true // Bool | Να επιστραφούν επίσης γεγονότα από ενοικιαστές που διαχειρίζεται αυτός ο ενοικιαστής. (optional)

DefaultAPI.getAuditLogs(tenantId: tenantId, options: DefaultAPI.GetAuditLogsOptions(limit: limit, skip: skip, order: order, after: after, before: before, username: username, ip: ip, crudType: crudType, resourceName: resourceName, targetId: targetId, target: target, includeManagedTenants: includeManagedTenants)) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
[inline-code-end]