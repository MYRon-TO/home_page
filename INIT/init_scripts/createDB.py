import mysql.connector

host="localhost",
user="user",
passwd="passwd",
database="mydatabase"

mydb = mysql.connector.connect(
  host=host,
  user=user,
  passwd=passwd,
  database=database
)

sql = '''
'''
