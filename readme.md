# Git-Cleaner 

### Description
Executable to automatically clean local branches which are not on remote anymore. It will ignore "main", "master" 
and the currently active branch.

#### Why?
This tool is fixing a pain which arised for me during many small bugfixes and for each it's 
own branch adding up painfully in the local history. To fast and easy get rid of everything stale it
can be run.
<br /> 

### Usage
Copy the executable in the directory containing the git repository and run it. <br />
As soon as it runs it will display the different lists: 
- Local Branches
- Remote Branches
- Stale Branches (The Delta between Local and Remote. The branches which get deleted.)

After a confirmation with "y" it will proceed and clean up.
<br/> 

#### Hint:
This can be added in .gitignore and therefore just live locally in your directory, or be checked in to the repository 
and be used by everyone if the flow of git usage is the same for everyone.

### :warning: Danger :warning: 
> [!CAUTION] 
> This tool is going to delete all the branches (**enforced**) which are not found on remote, so be careful, to either be 
> active on local Branch which you dont want to be deleted or delete the branches by hand.
