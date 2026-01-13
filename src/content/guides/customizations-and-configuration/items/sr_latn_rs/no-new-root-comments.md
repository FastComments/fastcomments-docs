[related-parameter-start name = 'noNewRootComments'; type = 'boolean'; related-parameter-end]

Postavljanje `noNewRootComments` na `true` će uzrokovati da widget sakrije polje za odgovore na najvišem nivou, ali i dalje dozvoljava korisnicima da odgovaraju
na podkomentare. Na primer, možete ovo uslovno postaviti prilikom učitavanja stranice kako biste dozvolili samo nekim korisnicima da ostavljaju komentare najvišeg nivoa.

[code-example-start config = {noNewRootComments: true}; linesToHighlight = [6]; title = 'Prevent New Root Comments'; code-example-end]