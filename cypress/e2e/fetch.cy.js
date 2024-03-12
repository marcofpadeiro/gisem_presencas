describe('Fetch Presença', () => {
    it('fetch', function() {
        cy.visit('https://gisem.dei.estg.ipleiria.pt/login');

        cy.fixture('users').as('users').then((users) => {
            cy.get('input[name=username]').type(users[0].username);
            cy.get('input[name=password]').type(users[0].password);
            cy.get('[type=submit]').click();


            Cypress.on('uncaught:exception', (err, runnable) => {
                return false;
            });

            cy.get('a[href="https://gisem.dei.estg.ipleiria.pt/obterAulasMarcarPresenca"]');
        });
    });
})
