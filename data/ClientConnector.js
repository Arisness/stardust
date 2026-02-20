import net from 'net';

const PORT = 8080;
const HOST = "localhost";

class ClientConnector {

    constructor(){
    }

    send(data){
        const client = net.connect(PORT, HOST, () =>{
            console.log('Link start!')
            client.write(JSON.stringify(data));
            
        });
        client.on('data', (data) => {
            console.log('Server is barking');
            client.end()
            console.log(this.deserialize(data));
        })
        client.on('end', () => {
            console.log('Link out')
        })
    }x

/**
 * JSON params:
 *      path: class path
 *      class: class name to be called
 *      method: method to be called
 *      params: params needed
 */
    createJSON(path, className, method, ...params){
        const data = {
            path: path,
            class: className,
            method: method,
            param: params
        }
        return data;
    }

    serialize(data){
        return JSON.stringify(data);
    }

    deserialize(data){
        return JSON.parse(data);
    }
}

export default ClientConnector