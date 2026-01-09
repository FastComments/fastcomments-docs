### ID di broadcast

Vedrai che dovrai passare un `broadcastId` in alcune chiamate API. Quando riceverai eventi, ti verrà restituito questo ID, così saprai di ignorare l'evento se prevedi di applicare le modifiche in modo ottimistico sul client
(probabilmente vorrai farlo, perché offre la migliore esperienza). Passa un UUID qui. L'ID dovrebbe essere sufficientemente unico da non ripetersi due volte durante una sessione del browser.