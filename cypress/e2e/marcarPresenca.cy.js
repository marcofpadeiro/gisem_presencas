describe('Marcar Presença', () => {
    before(() => {
        cy.fixture('users').as('users');
    });

    it('Should login and marcar presença for each user', function() {
        this.users.forEach((user) => {
            cy.visit('https://gisem.dei.estg.ipleiria.pt/login');

            cy.get('input[name=username]').type(user.username);
            cy.get('input[name=password]').type(user.password);
            cy.get('[type=submit]').click();

            Cypress.on('uncaught:exception', (err, runnable) => {
                return false;
            });

            cy.visit('https://gisem.dei.estg.ipleiria.pt/obterAulasMarcarPresenca').then(() => {
                cy.get('button[class="btn btn-primary col-xs-12"]').click();
            });

            // Listen to the alert event
            cy.on('window:alert', (str) => {
                // assert message from alert
                expect(str).to.equal('Marcação de presença foi feita com sucesso.');
            });

            cy.get('a[href="https://gisem.dei.estg.ipleiria.pt/logout"]').click();

        });
    });
})
