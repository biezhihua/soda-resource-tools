docker build -t soda_clix . 
docker tag soda_clix:latest biezhihua521/soda_clix:v0.1.2   
docker push biezhihua521/soda_clix:v0.1.2   

docker tag soda_clix:latest biezhihua521/soda_clix:latest 
docker push biezhihua521/soda_clix:latest