use crate::raymod::*;
use std::cmp::Ordering;

#[derive(Debug, Clone)]
struct BoundingBox {
    min: Point,
    max: Point,
}

#[derive(Debug, Clone)]
struct BVHNode {
    root:AABB,
    Leaf:usize,
    left:Box<BVHNode>,
    right:Box<BVHnode>
}

impl BVHNode {
    pub fn new( ary:&mut vec<usize>,scene: &Scene)->bool {
        AABBSort(& ary);
        Leaf=Nil_Leaf;
        root=scene.objects[0].bound_box;
        match scene.objects.len() {
            1=>{Leaf=ary[0]},
            2=>{
                root=root.marge(scene.objects[1]);
                let mut up_ary=vec![];
                let mut down_ary=vec![];
                up_ary.push(ary[0]);
                down_ary.push(ary[1]);
                left  = Box::new(BVHNode.new(up_ary,&scene) );
                right = Box::new(BVHNode.new(down_ary,&scene) );
                for i:=1..ary.() {
                }
            },
             _=>{}
    }

constructor BVHnode.Create(ary:IntegerArray;sph:TList);
var
   upAry,DownAry:IntegerArray;
   i,len:integer;
begin
   AABBSort(ary);
   Leaf:=Nil_Leaf;
   root:=sphereclass(sph[ary[0]]).BoundBox;
    
  case High(Ary) of
    0:Leaf:=ary[0];//要素1
    1:begin
       Root:=MargeBoundBox(Root,SphereClass(sph[ary[1] ]).BoundBox);
       setLength(UpAry,1);
       SetLength(downAry,1);
       upAry[0]:=Ary[0];
       DownAry[0]:=Ary[1];
       Left:=BVHNode.Create(upAry,sph);
       right:=BVHNode.Create(DownAry,sph);
    end;
    else begin
      for i:=1 to high(ary)  do begin
        Root:=MargeBoundBox(Root,SphereClass(sph[ary[i] ]).BoundBox);
      end;
      len:=length(Ary) div 2;
      upAry:=Copy(Ary,0,len);
      DownAry:=Copy(Ary,len,length(Ary)-len);
       
      Left:=BVHNode.Create(UpAry,sph);
      right:=BVHNode.Create(DownAry,sph);
    end;
  end;
end;

function BVHnode.intersect(r:RayRecord):InterRecord;
var
   RIR,LIR:InterRecord;
   t:real;
begin
  result.isHit:=false;
  result.t:=INF;
  result.id:=0;
  if leaf<>Nil_Leaf then begin
     result.t:=SphereClass(sph[leaf]).intersect(r);
     if result.t<INF then begin
        result.id:=Leaf;
        result.isHit:=true;
     end;
     exit;
  end;
  
  if root.Hit(r,EPS,INF) then begin
     RIR:=Right.intersect(r);
     LIR:=Left.intersect(r);
     if (LIR.isHit or RIR.isHit) then begin
        if RIR.isHit then result:=RIR;
        if LIR.isHit then begin
           if RIR.isHit=false then
              result:=LIR
           else if RIR.t>LIR.t then
              result:=LIR;
        end;
     end;
  end
  else begin
    result.isHit:=false;
    result.t:=INF;
  end;
end;
