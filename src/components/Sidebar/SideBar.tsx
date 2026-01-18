import SideBarItem, { SideBarItemProps} from "./SideBarItem";
import tollRoadIcon from '../../assets/icons/toll-road.svg';
import tollRoadFilledIcon from '../../assets/icons/toll-road-filled.svg';

const navItems: SideBarItemProps[] = [
    {title: "Portagens", icon: tollRoadIcon, iconSelected: tollRoadFilledIcon},
    {title: "Portagens", icon: tollRoadIcon, iconSelected: tollRoadFilledIcon},
];

function SideBar() {



    return (
    <>
    <div className="fixed w-1/9 min-w-36 h-screen
    flex flex-col justify-center items-center
    bg-zinc-50 shadow-sm">
        { navItems.map( (item, index) => (
            <SideBarItem 
            key={ item.title + index}
            title={item.title} 
            icon={item.icon} 
            iconSelected={ item.iconSelected}  
            />
        ) )}
    </div>
    <div className="w-1/9 min-w-36 h-screen"></div>    
    </>
);
}

export default SideBar;