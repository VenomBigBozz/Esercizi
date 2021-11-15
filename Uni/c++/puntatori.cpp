#include <iostream>

using namespace std;

void puntatore()
{
    int num = 25;  // numero intero
    int k = 20;    // k è un patore a int
    int *p = &num; //dichiara patore a intero
    *p = 34;       //assegna 10 al dato pato da patore
    p = &k;        //assegna ad patore indirizzo di 20

    //funzione che stampa patore
    cout << "Il valore di *p è: " << *p << endl;
    cout << "Il valore di p è: " << p << endl;
}

void puntaArray()
{
    double v[] = {1.2, 10.7, 9.8}; //dichiara array di double
    cout << v << endl;             // stampa indirizzo di v
    cout << *v << endl;            // stampa il primo elemento di v
    cout << v[0] << endl;          // stampa il primo elemento di v

    double *ptr = v;        //dichiara puntatore a double
    cout << ptr[1] << endl; // stampa il secondo elemento di v
    cout << ptr[2] << endl; // stampa il terzo elemento di v

    //aritmetica di puntatori
    cout << ptr << endl;        //stampa indirizzo di ptr
    cout << *(ptr + 1) << endl; // stampa il secondo elemento di v
    cout << *(ptr + 2) << endl; // stampa il terzo elemento di v
    cout << *(v + 2) << endl;   // stampa il terzo elemento di v

    double w[] = {3.4, 6.7, 9.8};
    // ERRORE v = w;
}

void aritmeticaPuntaArray()
{
    int v[] = {1, 2, 3, 4, 5}; // array di interi
    int *ptr = v;              // puntatore a interi
    cout << *(++ptr) << endl;  // stampa il secondo elemento di v
    cout << *(--ptr) << endl;  // stampa il primo elemento di v
    cout << *(ptr++) << endl;  // stampa il primo elemento di v
    cout << *(ptr) << endl;    // stampa il secondo elemento di v

    //addizione,sottrazione e assegnamento
    int *ptr1 = v;               // puntatore al dato ''1''
    int *ptr2 = &v[4];           // puntatore al dato ''5''
    cout << *(ptr1 + 1) << endl; // stampa il secondo elemento di v
    cout << *(ptr2 - 1) << endl; //stampa il primo elemento di v
    ptr2 -= 2;                   // decrementa di 2 il puntatore
    cout << *ptr2 << endl;       //stampa il terzo elemento di v
    ptr1 += 1;                   // incrementa di 1 il puntatore
    cout << *ptr1 << endl;       //stampa il secondo elemento di v
    cout << ptr2 - ptr1 << endl; //stampa 1 (non è un dato)

    // ERRORE *(ptr+7)=90;
    // ERRORE v[4]=100;
}

int main()
{
    puntatore();
    puntaArray();
    aritmeticaPuntaArray();
    return 0;
}