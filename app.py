from robyn import Robyn
from sql_rs import insert_person, get_person

app = Robyn(__file__)

@app.get('/person/:id')
def get(request):
    path_params = request.path_params
    return get_person(int( path_params['id'] ))


@app.post('/person')
def insert(request):
    json = request.json()
    name = json['name']
    age = int(json['age'])
    return insert_person(name, age)

if __name__ == '__main__':
    app.start()
