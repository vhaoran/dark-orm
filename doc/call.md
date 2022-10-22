# call type
   let r = db.find_many::<Bean>(doc!{}).limit(1)
              .skip(1)   
              .sort("a,b,c")
              .exec().await;
   let r = db.find_one::<Bean>(doc!{},None).exec();

   let r = db.delete_one::<Bean>(doc!{}).exec();
   let r = db.delete_many::<Bean>(doc!{}).exec();

   let r = db.insert_one::<Bean>(bean:Bean).exec();
   let r = db.insert_many::<Bean>(list:Vec<Bean>).exec();

   let r = db.update::<Bean>(
       doc!{},
       doc!{
         "$set":{
           a: 1,
           b: 2
         }
       })
       .exec();

# typecal call
  db.select::<Bean>()
     .cols("a,b,c")
     .where(" a = ? and b = ? and c = ?")
     .bind(param_1)
     .bind(param_1) 
     .order_by()
     .limit()
     .skip()

  db.update(" a = ? ,b=?,c=?,d=?")
      .bind(param_1)
      .bind(param_1)
     .where(" a = ? and b = ? and c = ?")

  db.delete()
      .where(" a=? and b = ? and c =?")
      .bind(param_1)
      .bind(param_1)



     
     
      

