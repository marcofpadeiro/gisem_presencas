#!/bin/bash

sleep=5 # minutes

while true; do 
    npx cypress run --spec "cypress/e2e/fetch.cy.js" && npx cypress run --spec "cypress/e2e/marcarPresenca.cy.js" && exit 0
    sleep $((60*$sleep))
done

cd -
