/*
 * <p>@project_name: CConstraintSolver
 * </p>
 * <p>@author: luhan zhen
 * </p>
 * <p>@date:  2023/11/16 13:30
 * </p>
 * <p>@email: zhenlh20@mails.jlu.edu.cn 
 * </p>
 * <p>@version: 1.0
 * </p>
  * <p>@description: 
 * </p>
 */



use crate::constraint::constraint::ConstraintTrait;
use crate::variable::variable::VariableTrait;

pub struct Problem
 {
     name:String,
     constraints: Box<Vec<Box<dyn ConstraintTrait>>>,
     variables: Box<Vec<Box<dyn VariableTrait>>>
 }
 
 
 
 