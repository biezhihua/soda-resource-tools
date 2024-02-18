docker build -t soda_clix . 

docker tag soda_clix:latest biezhihua521/soda_clix:v0.1.4   
docker push biezhihua521/soda_clix:v0.1.4   

docker tag soda_clix:latest biezhihua521/soda_clix:latest 
docker push biezhihua521/soda_clix:latest