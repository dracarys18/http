use std::collections::HashMap;
#[derive(Debug)]
pub struct QueryString<'buf>{
    data: HashMap<&'buf str,Values<'buf>>,
}
#[derive(Debug)]
pub enum Values<'buf>{
    Single(&'buf str),
    Multiple(Vec<&'buf str>),
}
impl<'buf> QueryString<'buf>{
    pub fn get(&self,key:&str)->Option<&Values>{
        self.data.get(key)
    }
}
impl<'buf> From<&'buf str> for QueryString<'buf>{
    fn from(val: &'buf str) -> Self{
        let mut data = HashMap::new();
        for i in val.split("&"){
            let mut key = i;
            let mut valu = "";
            if let Some(j) = i.find("=") {
                key = &i[..j];
                valu = &i[j+1..];
            }
            data.entry(key)
            .and_modify(|existing: &mut Values| match existing{
                Values::Single(prev_value)=>{
                    *existing = Values::Multiple(vec![prev_value,valu]);
                },
                Values::Multiple(vec)=>vec.push(valu),
                    
            })
            .or_insert(Values::Single(valu));
        }
        QueryString{data}
    }
}
